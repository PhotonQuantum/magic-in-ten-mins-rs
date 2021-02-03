# 十分钟魔法练习：广义代数数据类型

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础，ADT

```rust
use compile_fail::compile_fail;
```

在ADT中可以构造出如下类型：

```rust
enum ExprFail {
    IVal(i64),
    BVal(bool),
    Add(Box<ExprFail>, Box<ExprFail>),
    Eq(Box<ExprFail>, Box<ExprFail>)
}
```

但是这样构造有个问题，很显然`BVal`是不能相加的，而这样的构造并不能防止构造出这样的东西：`Add(Box::new(BVal(true)), Box::new(BVal(false)))`。实际上在这种情况下ADT的表达能力是不足的。

一个比较显然的解决办法是给`Expr`添加一个类型参数用于标记表达式的类型。
由于 Rust 的 enum 不支持添加关联类型，我们需要使用 trait 的 associative types 来模拟这一特性。

```rust
trait Expr: Copy + Clone { type Backing; }

#[derive(Copy, Clone)]
struct IVal(i64);
#[derive(Copy, Clone)]
struct BVal(bool);
#[derive(Copy, Clone)]
struct Add<T1: Expr<Backing=i64>, T2: Expr<Backing=i64>>(T1, T2);
#[derive(Copy, Clone)]
struct Eq<B: Copy, T1: Expr<Backing=B>, T2: Expr<Backing=B>>(T1, T2);

impl Expr for IVal { type Backing = i64; }
impl Expr for BVal { type Backing = bool; }
impl<T1: Expr<Backing=i64>, T2: Expr<Backing=i64>> Expr for Add<T1, T2> { type Backing = i64; }
impl<B: Copy, T1: Expr<Backing=B>, T2: Expr<Backing=B>> Expr for Eq<B, T1, T2> { type Backing = bool; }
```

这样就可以避免构造出两个类型为`bool`的表达式相加，能构造出的表达式都是类型安全的。

```rust
#[test]
fn test_gadt() {
    let I1 = Eq(IVal(10), IVal(10));
    let I2 = Eq(BVal(true), BVal(true));
    let I3 = Eq(I1, I2);
    let v1 = Add(IVal(10), IVal(2));
    let v2 = Add(IVal(0), IVal(3));
    let v3 = Add(v1, v2);
    let v4 = Add(v3, v2);
}

#[compile_fail]
fn fail_gadt() {
    // I1-I3, v1-v4 omitted
    // This will never check
    let fail1 = Eq(I3, v4);
    // This won't either
    let fail2: Add<BVal, BVal> = Add(I1, I2);
}
```

需要注意到四个 struct (`IVal`, `BVal`, `Add`, `Eq`) 实现的 trait 并不是 `Expr<Backing=T>` 而是包含了 associative type 的 `Expr`，这和ADT并不一样。而这即模拟了广义代数数据类型（Generalized Algebraic Data Type, GADT）。

> 注：
>
> Rust 模拟 GADT 的实现较为繁琐并且不易于理解，建议参考[原版 Java 模拟实现](https://github.com/goldimax/magic-in-ten-mins/blob/main/doc/GADT.md)。
>
> 另可参考 [refl](https://docs.rs/refl/) 使用了 GAT 构造类型相等的证明来实现 GADT。