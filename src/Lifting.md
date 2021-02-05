# 十分钟魔法练习：提升

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础，HKT，Monad

```rust
use crate::Monad::Monad;
use compile_fail::compile_fail;
```

## 概念

提升（Lifting）指的是把一个通用函数变成容器映射函数的操作。

比如把 `fn(A) -> B` 变成 `fn(M<A>) -> M<B>` 就是一种提升操作。而由于被操作的函数有一个参数所以这个操作也叫 `lift1` 。

注意被提升的函数可以有不止一个参数，我们也可以把 `fn(A, B) -> C` 提升为 `fn(M<A>, M<B>) -> M<C>` 。这样两个参数的提升可以称为 `lift2` 。

同样，被提升的函数可以没有参数，这时候我们可以看成没有这个函数，也就是把 `A` 提升为 `M<A>` 。这样的提升可以称为 `lift0` 。实际上它也和 `Monad` 中的 `pure` 是同构的。

也就是说：

```rust
#[compile_fail]
fn lift0<A>(f: A) -> M<A> {
    unimplemented!()
}

#[compile_fail]
fn lift1<A, B>(f: impl FnOnce(A) -> B) -> impl FnOnce(M<A>) -> M<B> {
    unimplemented!()
}

#[compile_fail]
fn lift2<A, B, C>(f: impl FnOnce(A, B) -> C) -> impl FnOnce(M<A>, M<B>) -> M<C> {
    unimplemented!()
}
```

> 注：
>
> Rust 不支持高阶类型的多态，所以在实际定义时应提供 M 指代的具体类型才能编译。

## fmap

看到这个函数签名肯定有人会拍案而起：这不就是 fmap 么？

fmap is a lifting surly. 因为它符合 lifting 的函数签名，但是 lifting 并不一定是 fmap 。只要符合这样的函数签名就可以说是一个 lifting 。

比如对于 list 来说 `f -> x -> x.tail().map(f)` 也符合 lifting 的函数签名，但很显然它不是一个 `fmap` 函数。或者说很多改变结构的函数和 `fmap` 组合还是一个 lifting 函数。

## 除此之外呢

回到上面那个函数签名，里面有个非泛型的参数 `M` ，这个 `M` 可以是个泛型参数，可以是个包装器比如 `Maybe` ，也可以是个线性容器比如 `List` ，可以是个非线性的容器比如 `Set`
，甚至可以是抽象容器比如 `Function` 。

同时提升操作也可能对容器结构做出一些改变，尤其是对于多参函数的提升可能会对函数的参数做出一些组合。比如对于 `List` 来说 `lift2` 既可以是 `zipMap` 也可也是以 `f` 为操作的卷积。

> 注：
>
> 由于同样的理由，Rust 无法传入容器类型进行构造。

## liftM

对于 Monad 来说，存在一种通用的提升操作叫 `liftM` ，比如对于 `vec` 来说 `liftM2` 就是：

```rust
fn liftM2Vec<A: Copy, B: Copy, C: Copy>(f: impl Fn(A, B) -> C + Copy)
                                        -> impl Fn(Vec<A>, Vec<B>) -> Vec<C> {
    move |ma: Vec<A>, mb: Vec<B>| {
        mdo!(Vec<_>,
            a <- ma;
            b <- mb.clone();
            pure f(a, b)
        )
    }
}
```

> 注：
>
> ... 这太扭曲了，但是我实在是糊不出更好的实现，类型体操顶不住了
>
> 如果有看起来阳间一点（可以用比 Copy 更弱一点的 trait bound，或者修改 do macro 使得不用写这个 clone）
> 的实现，欢迎 PR

而对 `sum` 进行提升以后的函数输入 `[1, 2, 3]` 和 `[2, 3, 4]` 就会得到 `[3, 4, 5, 4, 5, 6, 5, 6, 7]` 。实际上就是对于任意两个元素组合操作。

```rust
#[test]
fn test_liftM2Vec() {
    let sum = |a: i64, b: i64| a + b;
    assert_eq!(liftM2Vec(sum)(vec![1, 2, 3], vec![2, 3, 4]),
               vec![3, 4, 5, 4, 5, 6, 5, 6, 7]);
}
```

再比如 `liftM5` 在 `Haskell` 中的表述为：

```haskell
liftM5 f ma mb mc md me = do
  a <- ma
  b <- mb
  c <- mc
  d <- md
  e <- me
  pure (f a b c d e)
```

也就是 `liftM[n]` 就相当于嵌套 `n` 层 `flatMap` 提取 `Monad` 中的值然后应用给被提升的函数。

## 娱乐：Rust 过程宏实现 Lifting

> 注：
>
> 这非常不魔法，反倒很工程很黑魔法（不过话说回来，在 Rust 里写魔法教程已经用到了无数的黑魔法了）
>
> 并且本段与本节内容几乎无关，只是一点点个人迷思，建议读者凭兴趣选读，懒得糊可以快进直接看实现

以下内容选自 CNCF 所属 TiDB Community 在 2020 Q4 的 Mentorship - Enum RFC 的申请 Coding Task。

Implement auto_vec procedural macro

This programming task is aimed to help you learn the basics of the TiKV coprocessor framework.

In this task, you’ll need to implement an `auto_vec` procedural macro, which will automatically generate a vectorized
version of a function.

For example, here we have an “add” function.

``` rust
#[auto_vec]
fn add(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    if let Some(a) = a {
        if let Some(b) = b {
            return a + b;
        }
    }
    return None;
}
```

The `auto_vec` procedural macro will automatically generate something like

``` rust
fn add_vec(a: Vec<Option<usize>>, b: Vec<Option<usize>>) -> Vec<Option<usize>>
```

And users may call `add_vec` to apply “add” on all elements of a Vec.

``` rust
let a = vec![Some(1), Some(2), Some(3), None];
let b = vec![Some(3), Some(2), None, Some(0)];
println!("{:?}", add_vec(a, b)); // [Some(4), Some(4), None, None]
```

You’ll need to create an `auto_vec` procedural macro for functions of any number of arguments.
You’ll need to report a runtime error if the parameters of the vectorized functions are not correct.
(e.g. a and b have different length)

注意到，以上题目要求本质上是要求将一个 `A -> B -> ... -> Z` 的函数提升为 `Vec<A> -> Vec<B> -> ... -> Vec<Z>`。
只不过其使用的技术手段并不是 lift 函数，而是通过 Rust 的过程宏 codegen 出一个新的函数。

并且请仔细观察题设，其要求的函数体行为与 liftM 存在不同。

以下给出需要通过的单测项目

``` rust
// Compile error
#[auto_vec]
fn fn_1() -> String;

// Compile error
#[auto_vec]
fn fn_2(x: usize);

// trivial test case
#[auto_vec]
fn add(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    if let Some(a) = a {
        if let Some(b) = b {
            return Some(a + b);
        }
    }
    return None;
}

// handle generics
#[auto_vec]
fn fn_3<X: Into<usize>, Y: Into<usize>>(
    a: X,
    b: Y,
    c: String
) -> usize {
    a.into() + b.into()
}

struct Location {
    x: i64,
    y: i64,
}

// handle arguments with auto-unpacking
#[auto_vec]
fn fn_4((Location { x, .. }, Location { y, .. }, Location { x: x2, .. }): (Location, Location, Location)) -> i64 {
    x * y * x2
}

#[test]
fn autovec_test() {
    let a = vec![Some(1), Some(2), Some(3), None];
    let b = vec![Some(3), Some(2), None, Some(0)];
    assert_eq!(add_vec(a, b), [Some(4), Some(4), None, None]);

    let a = vec![
        (Location { x: 1, y: 2 }, Location { x: 4, y: 9 }, Location { x: 9, y: 7 }),
        (Location { x: 3, y: 2 }, Location { x: 4, y: 1 }, Location { x: 4, y: 7 }),
        (Location { x: 2, y: 2 }, Location { x: 4, y: 2 }, Location { x: 5, y: 7 })
    ];
    assert_eq!(a, vec![81, 12, 20]);
}
```

~~The implementation of this proc macro is left as an exercise to the readers.~~

[参考实现](https://github.com/PhotonQuantum/autovec-impl-rs)