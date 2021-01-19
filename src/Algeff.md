# 十分钟魔法练习：代数作用

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础，续延

```rust
use std::fmt::{self, Display, Formatter};
use std::error::Error;
```

## 可恢复异常

有时候我们希望在异常抛出后经过保存异常信息再跳回原来的地方继续执行。

参考 [延续](Continuation.md) 一节中 `try-catch` 的实现，
如果我们有了异常抛出时的续延那么可以带有 `resume` 块， 在 `catch` 块中调用这个续延就能恢复之前的执行状态。

下面是实现可恢复异常的 `try-catch` ：

```rust
type BareResumeFuncTy = dyn FnOnce() -> ();
type FinalFuncTy = dyn FnOnce() -> ();
type BareCatchFuncTy = dyn FnOnce(Box<dyn Error>, Box<BareResumeFuncTy>) -> ();
type BareBodyFuncTy = dyn FnOnce(Box<BareCatchFuncTy>, Box<FinalFuncTy>) -> ();

fn r#try(body: Box<BareBodyFuncTy>, catch: Box<BareCatchFuncTy>, r#final: Box<FinalFuncTy>) {
    body(catch, r#final);
}
```

然后就可以像下面这样使用：

```rust
#[derive(Debug, Copy, Clone)]
struct ZeroDivision;

impl Display for ZeroDivision {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "divide by zero")
    }
}

impl Error for ZeroDivision {}

fn try_div_resume(t: u64) {
    r#try(
        Box::new(move |throw, r#final| {
            println!("try");
            if t == 0 {
                throw(Box::new(ZeroDivision), Box::new(|| {
                    println!("resumed");
                    r#final();
                }));
            } else {
                println!("{}", 100 / t);
                r#final();
            }
        }),
        Box::new(|e, cont| {
            println!("catch {:#?}", e);
            cont();
        }),
        Box::new(|| println!("final")),
    );
}

#[test]
fn test_try_resume() {
    try_div_resume(0);
}
```

而调用 `try_div_resume(0)` 就会得到：

```
try
catch ZeroDivision
resumed
final
```

## 代数作用

如果说在刚刚异常恢复的基础上希望在恢复时修补之前的异常错误就需要把之前的 `resume` 函数加上参数，
这样修改以后它就成了代数作用（Algebraic Effect）的基础工具：

```rust
type ResumeFuncTy<T> = dyn FnOnce(T) -> ();
type CatchFuncTy<T> = dyn FnOnce(Box<dyn Error>, Box<ResumeFuncTy<T>>) -> ();
type BodyFuncTy<T> = dyn FnOnce(Box<CatchFuncTy<T>>, Box<FinalFuncTy>) -> ();

fn try_alt<T>(body: Box<BodyFuncTy<T>>, catch: Box<CatchFuncTy<T>>, r#final: Box<FinalFuncTy>) {
    body(catch, r#final);
}
```

使用方式如下：

```rust
fn try_div_resume_alt(t: u64) {
    try_alt(
        Box::new(move |throw, r#final| {
            println!("try");
            if t == 0 {
                throw(Box::new(ZeroDivision), Box::new(|v: u64| {
                    println!("resumed {}", 100 / v);
                    r#final();
                }));
            } else {
                println!("{}", 100 / t);
                r#final();
            }
        }),
        Box::new(|e, cont| {
            println!("catch {:#?}", e);
            cont(1);
        }),
        Box::new(|| println!("final")),
    );
}

#[test]
fn test_try_resume_alt() {
    try_div_resume_alt(0);
}
```

而这个东西能实现不只是异常的功能，从某种程度上来说它能跨越函数发生作用（Perform Effect）。

比如说现在有个函数要记录日志，但是它并不关心如何记录日志，输出到标准流还是写入到文件或是上传到数据库。
这时候它就可以调用

``` rust
perform(log_it(INFO, "test"), ...);
```

来发生（Perform）一个记录日志的作用（Effect）然后再回到之前调用的位置继续执行，
而具体这个作用产生了什么效果就由调用这个函数的人实现的 `try` 中的 `handler` (`catch`) 决定。
这样发生作用和执行作用（Handle Effect）就解耦了。

进一步讲，发生作用和执行作用是可组合的。对于需要发生记录日志的作用，
可以预先写一个输出到标准流的的执行器（Handler）一个输出到文件的执行器然后在调用函数的时候按需组合。
这也就是它是代数的（Algebraic）的原因。

细心的读者还会发现这个东西还能跨函数传递数据，在需要某个量的时候调用

```java
perform(ask("config"), ...);
```

就可以获得这个量而不用关心这个量是怎么来的，内存中来还是读取文件或者 HTTP 拉取。从而实现获取和使用的解耦。

而且这样的操作和状态单子非常非常像，实际上它就是和相比状态单子来说没有修改操作的读取器单子（Reader Monad）同构。

也就是说把执行器函数作为读取器单子的状态并在发生作用的时候执行对应函数就可以达到和用续延实现的代数作用相同的效果，
反过来也同样可以模拟。