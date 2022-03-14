# 十分钟魔法练习：高阶类型

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础

```rust
use compile_fail::compile_fail;
```

> 注意：
> 
> 本节使用了 generic associated types 这一不稳定语言特性，需要使用 Nightly Rust 进行编译，并请注意其实现仍然存在问题，可能会造成编译器内部错误或者异常行为，因此请不要将其应用于生产环境。
> 
> HKT 在 Stable Rust 中也可以进行模拟，但是其写法不易于理解。感兴趣的读者可以自行阅读 [Method for Emulating Higher-Kinded Types in Rust](https://gist.github.com/edmundsmith/855fcf0cb35dd467c29a9350481f0ecf) 的实现。

## 常常碰到的困难

写代码的时候常常会碰到语言表达能力不足的问题，比如下面这段用来给`F`容器中的值进行映射的代码：

```rust
#[compile_fail]
fn fail_functor() {
    trait Functor<A> {
        fn map<F, B>(&self, f: F) -> Self<B> where F: Fn(&A) -> B;
        //                                ^ type argument not allowed
    }
}
```

并不能通过编译。

## 高阶类型

假设类型的类型是`Type`，比如`int`和`string`类型都是`Type`。

而对于`Vec`这样带有一个泛型参数的类型来说，它相当于一个把类型`T`映射到`Vec<T>`的函数，其类型可以表示为`Type -> Type`。

同样的对于`Map`来说它有两个泛型参数，类型可以表示为`(Type, Type) -> Type`。

像这样把类型映射到类型的非平凡类型就叫高阶类型（HKT, Higher Kinded Type）。

虽然 Rust 中存在这样的高阶类型，但是我们并不能用一个泛型参数表示出来，也就不能写出如上`Self<A>`这样的代码了，因为`Self: impl Functor`是个高阶类型。

> 如果加一层解决不了问题，那就加两层。

虽然在 Rust 中不能直接表示出高阶类型，但是我们可以通过加一个中间层来在保留完整信息的情况下强类型地模拟出高阶类型。

首先，我们需要一个中间层来储存高阶类型信息：

```rust
pub trait HKT {
    type Higher<T>;
}
```

然后我们就可以用 `Higher<A>` 来表示 `F<A>` ，这样操作完 `Higher<A>` 后我们仍然有完整的类型信息来还原 `F<A>` 的类型。

这样，上面`Functor`就可以写成：

```rust
trait Functor<A>: HKT {
    fn map<F, B>(&self, f: F) -> Self::Higher<B> where F: Fn(&A) -> B;
}
```

这样就可以编译通过了。而对于想实现`Functor`的类，需要先实现`HKT`这个中间层，这里拿`Vec`举例：

```rust
impl<A> HKT for Vec<A> {
    type Higher<T> = Vec<T>;
}
```

这样，实现`Functor`类就是一件简单的事情了：

```rust
impl<A> Functor<A> for Vec<A> {
    fn map<F, B>(&self, f: F) -> Self::Higher<B> where F: Fn(&A) -> B {
        self.iter().map(f).collect()
    }
}
```

```rust
#[test]
fn test_hkt() {
    let test_vec: Vec<i64> = vec![1, 2, 4, 3, 6];
    let result = test_vec.map(|num|{num % 2 == 0});
    assert_eq!(result, vec![false, true, true, false, true]);
}
```
