# 十分钟魔法练习：单子

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础，HKT

```rust
use crate::HKT::HKT;
use compile_fail::compile_fail;
```

> 注意：
>
> 本节依赖的 HKT 使用了不稳定语言特性实现，详见 [高阶类型](HKT.md) 一节。

## 单子

单子(Monad)是指一种有一个类型参数的数据结构，拥有`pure`（也叫`unit`或者`return`）和`fmap`（也叫`bind`或者`>>=`）两种操作：

```rust
trait Monad<A>: HKT {
    fn pure(v: A) -> Self;
    fn fmap<F, B>(&self, f: F) -> Self::Higher<B> where F: Fn(&A) -> Self::Higher<B>;
}
```

其中`pure`要求返回一个包含参数类型内容的数据结构，`fmap`要求把值经过`f`以后再串起来。

举个最经典的例子：

## List Monad

```rust
impl<A> Monad<A> for Vec<A> {
    fn pure(v: A) -> Self {
        vec![v]
    }
    fn fmap<F, B>(&self, f: F) -> Self::Higher<B> where F: Fn(&A) -> Self::Higher<B> {
        self.into_iter().flat_map(f).collect()
    }
}
```

于是我们可以得到如下平凡的结论：

```rust
#[test]
fn test_monad_vec() {
    assert_eq!(Vec::<i64>::pure(3), vec![3]);
    assert_eq!(vec![1, 2, 3].fmap(|v| vec![v + 1, v + 2]), vec![2, 3, 3, 4, 4, 5])
}
```

## Option Monad

Rust 是一个空安全的语言，想表达很多语言中`null`这一概念，我们需要使用 `Option` 类型。对于初学者来说，面对一串可能出现空值的逻辑来说，判空常常是件麻烦事：

```rust
fn add_i(ma: Option<i64>, mb: Option<i64>) -> Option<i64> {
    // 提示：请不要在实际场景下写出这样的代码
    if let None = ma { return None; }
    if let None = mb { return None; }
    Some(ma.unwrap() + mb.unwrap())
}
```

现在，我们将`Option`扩展成`HKT`：

```rust
impl<A> HKT for Option<A> {
    type Higher<T> = Option<T>;
}
```

可以像这样定义`Option Monad`：

```rust
impl<A> Monad<A> for Option<A> {
    fn pure(v: A) -> Self {
        Some(v)
    }
    fn fmap<F, B>(&self, f: F) -> Self::Higher<B> where F: Fn(&A) -> Self::Higher<B> {
        if let None = self {
            None
        } else {
            f(self.as_ref().unwrap())
        }
    }
}
```

上面`add_i`的代码就可以改成：

```rust
fn add_m(ma: Option<i64>, mb: Option<i64>) -> Option<i64> {
    type m = Option::<i64>;
    ma.fmap(|a| {             // a <- ma
        mb.fmap(|b| {         // b <- mb
            m::pure(a + b)    // pure (a + b)
        })
    });
    
    // 也可以写成如下形式
    m::fmap(&ma, |a|{    // a <- ma
    m::fmap(&mb, |b|{    // b <- mb
    m::pure(a+b)         // pure (a + b)
    })})
}
```

这样看上去比连续`if-return`优雅很多。在一些有语法糖的语言(`Haskell`)里面Monad的逻辑甚至可以像上面右边的注释一样简单明了。

敏锐的读者在阅读上述`add_i`的实现时，可能已经发现，在 Rust 中这一函数有更自然的内置写法：

```rust
fn add_i_alt(ma: Option<i64>, mb: Option<i64>) -> Option<i64> {
    ma.and_then(|a|{
        mb.and_then(|b|{
            Some(a + b)
        })
    })
}
```

如果你曾阅读过 Option 相关的文档或源码，你可能会看到如下注释：

```rust
/// Returns [`None`] if the option is [`None`], otherwise calls `f` with the
/// wrapped value and returns the result.
///
/// Some languages call this operation flatmap.
#[compile_fail]
pub fn and_then<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<U> {}
```

让我们抄下来并改写一下 fmap 的签名：
```rust
    #[compile_fail]
    fn fmap<U, F: Fn(&A) -> Self::Higher<U>>(&self, f: F) -> Self::Higher<U> {}
```

所以，如果你已经写过一段时间的 Rust，你很有可能已经在不知不觉中大量使用了 Option Monad 和 flatmap 了。