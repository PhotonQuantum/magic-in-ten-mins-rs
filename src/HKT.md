# 十分钟魔法练习：高阶类型

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础

```rust
use compile_fail::compile_fail;
```

## 常常碰到的困难

写代码的时候常常会碰到语言表达能力不足的问题，比如下面这段用来给`F`容器中的值进行映射的代码：

```rust
#[compile_fail]
fn fail_functor() {
    trait Functor {
        fn map<F, A, B>(&self, f: F) -> Self<B> where F: Fn(&A) -> B;
        //                                   ^ type argument not allowed
    }
}
```

并不能通过编译。

## 高阶类型

假设类型的类型是`Type`，比如`int`和`string`类型都是`Type`。

而对于`Vec`这样带有一个泛型参数的类型来说，它相当于一个把类型`T`映射到`Vec<T>`的函数，其类型可以表示为`Type -> Type`。

同样的对于`Map`来说它有两个泛型参数，类型可以表示为`(Type, Type) -> Type`。

> 注：需要注意本文对 Map 的定义和 Rust 标准库中的定义有所不同，Rust 将 Map 定义为 Map<I, F>，其中 I 代表迭代器，F 代表映射函数。
> 
> 下文中默认以 Map<A, B> 为准。

像这样把类型映射到类型的非平凡类型就叫高阶类型（HKT, Higher Kinded Type）。

虽然 Rust 中存在这样的高阶类型，但是我们并不能用一个泛型参数表示出来，也就不能写出如上`Self<A>`这样的代码了，因为`Self: impl Functor`是个高阶类型。

> 如果加一层解决不了问题，那就加两层。

虽然在 Rust 中不能直接表示出高阶类型，但是我们可以通过加一个中间层来在保留完整信息的情况下强类型地模拟出高阶类型。

首先，我们需要一个中间层：

```rust
trait HKT<B> {  // B is the new inner type
    type A;     // A is the current type
    type M;     // M is the new type - F<B>
}               // F<A>
```

> 注：这个定义比较反直觉，但是这是我能想到的在 Rust 里简单表达 HKT 的唯一方法。有空的话我会补上另一种用宏模拟 HKT 的黑魔法方法。

这样，上面`Functor`就可以写成：

```rust
trait Functor<B>: HKT<B> {
    fn map<F>(&self, f: F) -> Self::M where F: Fn(&Self::A) -> B;
}
```

这样就可以编译通过了。而对于想实现`Functor`的类，需要先实现`HKT`这个中间层，这里拿`Vec`举例：

```rust
impl<A, B> HKT<B> for Vec<A> {
    type A = A;
    type M = Vec<B>;
}
```

这样，实现`Functor`类就是一件简单的事情了：

```rust
impl<A, B> Functor<B> for Vec<A> {
    fn map<F>(&self, f: F) -> Self::M where F: Fn(&Self::A) -> B{
        self.into_iter().map(f).collect()
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

> 注：这种定义方法对于理解 HKT 可能没什么好处，建议参考[原版](https://github.com/goldimax/magic-in-ten-mins/blob/main/doc/HKT.md)。
