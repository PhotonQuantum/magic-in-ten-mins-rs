# 十分钟魔法练习：续延

### By 「玩火」 改写 「光量子」

> 前置技能：简单 Rust 基础

```rust
use std::fmt::{self, Formatter, Display};
use std::error::Error;
```

## 续延

续延（Continuation）是指代表一个程序未来的函数，其参数是一个程序过去计算的结果。

比如对于这个程序：

```rust
fn _test() {
    let mut i: u64 = 1;         // 1
    i += 1;                     // 2
    println!("{}", i);          // 3
}
```

它第二行以及之后的续延就是：

```rust
fn _cont2(mut i: u64) {
    i += 1;                     // 2
    println!("{}", i);          // 3
}
```

而第三行之后的续延是：

```rust
fn _cont3(i: u64) {
    println!("{}", i);      // 3
}
```

实际上可以把这整个程序的每一行改成一个续延然后用函数调用串起来变成和刚才的程序一样的东西：

```rust
fn cont1() {
    let i: u64 = 1;         // 1
    cont2(i);
}

fn cont2(mut i: u64) {
    i += 1;                 // 2
    cont3(i);
}

fn cont3(i: u64) {
    println!("{}", i);      // 3
}

fn test() {
    cont1();
}
```

## 续延传递风格

续延传递风格（Continuation-Passing Style, CPS）是指把程序的续延作为函数的参数来获取函数返回值的编程思路。

听上去很难理解，把上面的三个 `cont` 函数改成CPS就很好理解了：

```rust
fn logic1(f: impl Fn(u64)) {
    let i: u64 = 1;
    f(i);   // return i
}
fn logic2(mut i: u64, f: impl Fn(u64)) {
    i += 1;
    f(i);
}
fn logic3(i: u64, f: impl Fn(u64)) {
    println!("{}", i);
    f(i);
}
fn test_cont() {
         logic1(        // 获取返回值 i
    move |i| logic2(i, 
    move |i| logic3(i, 
    move |i| {})));
}
```

每个 `logic` 函数的最后一个参数 `f` 就是整个程序的续延，而在每个函数的逻辑结束后整个程序的续延也就是未来会被调用。而 `test` 函数把整个程序组装起来。

读者可能已经注意到，`test_cont` 函数写法很像 Monad。实际上这个写法就是 Monad 的写法， Monad 的写法就是 CPS。

另一个角度来说，这也是回调函数的写法，每个 `logic` 函数完成逻辑后调用了回调函数 `f` 来完成剩下的逻辑。实际上，异步回调思想很大程度上就是 CPS 。

> 注：
> 
> 个人理解所有的 CPS 应该都可以被改写成 Monad，而 Monad 调整一下类型应该也可以改写成 CPS。

## 有界续延

考虑有另一个函数 `call_t` 调用了 `test` 函数，如：

```rust
fn call_t() {
    test();
    println!("3");
}
```

那么对于 `logic` 函数来说调用的 `f` 这个续延并不包括 `call_t` 中的打印语句，那么实际上 `f` 这个续延并不是整个函数的未来而是 `test` 这个函数局部的未来。

这样代表局部程序的未来的函数就叫有界续延（Delimited Continuation）。

实际上在大多时候用的比较多的还是有界续延，因为在获取整个程序的续延还是比较困难的，这需要全用 CPS 的写法。

## 异常

拿到了有界续延我们就能实现一大堆控制流魔法，这里拿异常处理举个例子，通过CPS写法自己实现一个 `try-throw` 。

首先最基本的想法是把每次调用 `try` 的 `catch` 函数保存起来，由于 `try` 可层层嵌套所以每次压入栈中，然后 `throw` 的时候将最近的 `catch` 函数取出来调用即可

> 注：
> 
> 这边为了规避全局的 vector 并简化所有权与引用关系，使用了 call stack 来持有 catch handler，效果是一样的。

```rust
type FinalFuncTy = dyn FnOnce() -> ();
type CatchFuncTy = dyn FnOnce(Box<dyn Error>, Box<FinalFuncTy>) -> ();
type BodyFuncTy = dyn FnOnce(Box<CatchFuncTy>, Box<FinalFuncTy>) -> ();

fn r#try(body: Box<BodyFuncTy>, catch: Box<CatchFuncTy>, r#final: Box<FinalFuncTy>) {
    body(catch, r#final);
}
```

这里 `body` 的参数和 `catch` 的最后一个参数都是有界续延。

有了 `try-throw` 就可以按照CPS风格调用它们来达到处理异常的目的：

```rust
#[derive(Debug, Copy, Clone)]
struct ZeroDivision;

impl Display for ZeroDivision {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "divide by zero")
    }
}

impl Error for ZeroDivision {}

fn try_div(t: u64) {
    r#try(
        Box::new(move |throw, r#final| {
            println!("try");
            if t == 0 {
                throw(Box::new(ZeroDivision), r#final);
            } else {
                println!("{}", 100 / t);
                r#final();
            }
        }),
        Box::new(|e, r#final| {
            println!("catch {:#?}", e);
            r#final();
        }),
        Box::new(|| println!("final")),
    );
}

#[test]
fn test_try() {
    try_div(1);
    try_div(0);
}
```

调用 `try_div(0)` 会得到：

```
try
catch ZeroDivision
final
```

而调用 `try_div(1)` 会得到：

```
try
100
final
```
