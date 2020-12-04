# 十分钟魔法练习：单位半群

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础

```rust
use self::Ordering::*;
```

## 半群（Semigroup）

半群是一种代数结构，在集合 `A` 上包含一个将两个 `A` 的元素映射到 `A` 上的运算即 `<> : (A, A) -> A` ，同时该运算满足**结合律**即 `(a <> b) <> c == a <> (b <> c)` ，那么代数结构 `{<>, A}` 就是一个半群。

比如在自然数集上的加法或者减法可以构成一个半群，再比如字符串集上字符串的连接构成一个半群。

用 Rust 代码可以表示为：

```rust
trait Semigroup: Sized {
    fn op(self, other: Self) -> Self;
}
```

## 单位半群（Monoid）

单位半群是一种带单位元的半群，对于集合 `A` 上的半群 `{<>, A}` ，`A`中的元素`a`使`A`中的所有元素`x`满足 `x <> a` 和 `a <> x` 都等于 `x`，则 `a` 就是 `{<>, A}` 上的单位元。

> 注：单位半群有另一个常用的名字叫“幺半群”，其中幺作数字一之解。

举个例子，`{+, 自然数集}`的单位元就是0，`{*, 自然数集}`的单位元就是1，`{+, 字符串集}`的单位元就是空串`""`。

用 Rust 代码可以表示为：

```rust
trait Monoid: Semigroup {
    fn id() -> Self;
}

impl<T: Default + Semigroup> Monoid for T {
    fn id() -> Self {
        T::default()
    }
}
```
> 提示：
> 
> 可以注意到，rust 的 `Default` trait 提供给我们的 default 值正是我们想要的单位元。
> 方便起见，上述定义中将自动从 default 中取到单位元，并自动将半群定义成单位半群。

## 应用：Option

在 Rust 中有类型`Option`可以用来表示可能有值的类型，而我们可以将它定义为 Monoid：

```rust
impl<T> Semigroup for Option<T> {
    fn op(self, b: Self) -> Self {
        if self.is_some() { self } else { b }
    }
}
// Option<T> 已经实现 Default trait，此时将自动被定义成单位半群。
```

这样对于 ops 来说我们将获得一串 Option 中第一个不为空的值，对于需要进行一连串尝试操作可以这样写：

```rust
#[test]
fn test_monoid_option() {
    let result = Option::<isize>::id().op(None).op(Some(2)).op(Some(3)).op(None);
    assert_eq!(result, Some(2));
}
```

## 应用：Ordering

可以利用 Monoid 实现带优先级的比较

```rust
#[derive(Debug, PartialEq)]
enum Ordering {
    Lt,
    Eq,
    Gt
}
fn compare_str(a: &str, b: &str) -> Ordering {
    if a < b { Lt } else if a > b { Gt } else { Eq }
}
```

```rust
impl Semigroup for Ordering {
    fn op(self, other: Self) -> Self {
        if self == Eq { other } else { self }
    }
}

impl Monoid for Ordering {
    fn id() -> Self { Eq }
}
```

同样如果有一串带有优先级的比较操作就可以用appends串起来，比如：

```rust
#[derive(Copy, Clone)]
struct Student<'a> {
    name: &'a str,
    sex: &'a str,
    from: &'a str
}

impl Student<'_> {
    fn compare(&self, other: &Student) -> Ordering {
        Ordering::id()
            .op(compare_str(&self.name, &other.name))
            .op(compare_str(&self.sex, &other.sex))
            .op(compare_str(&self.from, &other.from))
    }
}
```

这样的写法比一连串`if-else`优雅太多。

```rust
#[test]
fn test_monoid_ordering() {
    let student_1 = Student { name: "Alice", sex: "Female", from: "Utopia" };
    let student_2 = Student { name: "Dorothy", sex: "Female", from: "Utopia" };
    let student_3 = Student { name: "Alice", sex: "Female", from: "Vulcan" };
    assert_eq!(student_1.compare(&student_2), Lt);
    assert_eq!(student_3.compare(&student_1), Gt);
    assert_eq!(student_1.compare(&student_3), Lt);
    assert_eq!(student_1.compare(&student_1), Eq);
}
```

## 扩展

这部分代码使用了 Java 的 `Runnable`，而这在 Rust 中并没有很好的直接对应或替代，
建议参考[原版](https://github.com/goldimax/magic-in-ten-mins/blob/main/doc/Monoid.md#%E6%89%A9%E5%B1%95)。

> 注：上面 Option 的实现并不是 lazy 的，实际运用中加上非空短路能提高效率。