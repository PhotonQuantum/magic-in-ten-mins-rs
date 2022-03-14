# 十分钟魔法练习：代数数据类型 (ADT)

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础

```rust
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use self::Bool::*;
use self::List::*;
use self::Nat::*;
```

## 积类型（Product type）

积类型是指同时包括多个值的类型，比如 Rust 中的 struct 就会包括多个字段：

```rust
struct Student {
    name: String,
    id: i64,
}
```

而上面这段代码中 Student 的类型中既有 String 类型的值也有 isize 类型的值。这种情况我们称其为 String 和 isize 的「积」，即`String * i64`。

## 和类型（Sum type）

和类型是指可以是某一些类型之一的类型，在 Rust 中可以用 enum 来表示：

```rust
enum SchoolPerson {
    Student { name: String, id: i64 },
    Teacher { name: String, office: String },
}
```

SchoolPerson 可能是 Student 也可能是 Teacher。这种类型存在多种“变体”的情形，我们称之为 Student 和 Teacher 的「和」，即`String * isize + String * String`。使用时可以通过 Pattern Matching 知道当前的 StudentPerson 具体是 Student 还是 Teacher。

## 代数数据类型（ADT, Algebraic Data Type）

由和类型与积类型组合构造出的类型就是代数数据类型，其中代数指的就是和与积的操作。

### 布尔类型

利用和类型的枚举特性与积类型的组合特性，我们可以构造出 Rust 中本来很基础的基础类型，比如枚举布尔的两个量来构造布尔类型：

```rust
enum Bool {
    True,
    False,
}
```

模式匹配可以用来判定某个 Bool 类型的值是 True 还是 False。

```rust
#[test]
fn test_bool() {
    let b = True;
    match b {
        True => (),
        False => panic!("oh my?"),
    };
}
```

### 自然数

让我们看一些更有趣的结构。我们知道，一个自然数要么是 0，要么是另一个自然数 +1。如果理解上有困难，可以将其看作是一种“一进制”的计数方法。这种自然数的构造法被称为皮亚诺结构。利用 ADT，我们可以轻易表达出这种结构：

```rust
enum Nat {
    S(Box<Nat>),
    O,
}
macro_rules! S {
    ($n: expr) => {
        S(Box::new($n))
    };
}
```

其中，`O` 表示自然数 0，而 `S` 则代表某个自然数的后继（即+1）。例如，3 可以用`S!(S!(S!(O)))`来表示。

```rust
impl Display for Nat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let n = std::iter::successors(Some(self), |n| match n {
            S(n) => Some(&*n),
            O => None,
        }).skip(1).count();
        write!(f, "{}", n)
    }
}

#[test]
fn test_nat() {
    let nat = S!(S!(S!(O)));
    assert_eq!(format!("{}", nat), String::from("3"));
}
```

> 读者可能会注意到，我们使用了`Box<Nat>`而不是`Nat`来表达 `S`，这是由于在 Rust 中，所有栈上的数据结构的空间大小（内存占用）必须在编译期决定。如果我们将 `S` 定义为 `S(Nat)`，则以下两个表达式都是 `Nat` 类型的合法实例：`S(O)`, `S(S(S(O)))`。显然，这两个值在内存中占用的长度不一致。在最糟糕的情况下，由于自然数大小没有上界，其大小甚至可能是无穷的。这说明如果如此定义 `Nat`，我们的编译器将不知道它具体要占用多大的空间。因此，我们使用了`Box`这一智能指针类型，将内嵌的`Nat`放到堆内存中，那么`Nat`在栈上的大小即可确定了。
>
>  为了降低阅读的困难程度，我们使用了宏来简化其表达，以下不再赘述。

### 链表

```rust
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
impl<T> Default for List<T> {
    fn default() -> Self {
        Nil
    }
}
#[macro_export]
macro_rules! Cons {
    ($n: expr, $l: expr) => {
        List::Cons($n, Box::new($l))
    };
}
```

`[1, 3, 4]`可以被表示为 `Cons!(1, Cons!(3, Cons!(4, Nil)))`

## 何以代数?

代数数据类型之所以被称为“代数”，是因为其可以像代数一样进行运算。其实，每种代数数据类型都对应着一个值，即这种数据类型可能的实例数量。

显然，积类型的实例数量来自各个字段可能情况的组合，也就是各字段实例数量相乘。而和类型的实例数量，就是各种可能类型的实例数量之和。

例如，`Bool`的实例只有`True`和`False`两种情况，其对应的值就是`1+1`。而`Nat`除了最初的`O`以外，对于每个`Nat`值`n`都存在`S(n)`，其也是`Nat`类型的值。那么，我们可以将`Nat`对应到`1+1+1+...`，其中每一个 1 都代表一个自然数。至于 List 的类型就是`1+x(1+x(...))`也就是`1+x^2+x^3...`其中 `x `就是 List 所存类型的实例数量。

> 容易注意到，上述的`Nat`与`List`类型，它们的合法实例数量都是无穷的。那么在之前定义数据结构时，如果不使用指针则其空间大小不可确定，即是平凡的事情了。

到现在为止，我们已经通过代数数据类型粗略定义出了加法与乘法。其实，我们还可以定义出零值以及指数计算。另外，加法的交换率等定理可以通过这套类型系统进行证明。感兴趣的读者可以查询相关资料，进一步进行探究。

## 实际运用

ADT 最适合构造树状的结构，比如解析 JSON 出的结果需要一个聚合数据结构。

```rust
enum JsonValue {
    Bool(bool),
    Int(i64),
    String(String),
    Array(Vec<JsonValue>),
    Map(HashMap<String, JsonValue>),
}
```
