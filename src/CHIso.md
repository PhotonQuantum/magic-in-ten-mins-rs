# 十分钟魔法练习：Curry-Howard 同构

### By 「玩火」 改写 「光量子」

> 前置技能：构造演算
> 
> Rust 部分：Rust 基础，命题逻辑

## 记忆碎片

我初中刚学几何证明的时候想过一个问题，能否用计算机来自动批改证明。那时候我还在用 VB 语言，能想到的办法也就只有字符串匹配替换。比如说下面的证明：

```
已知: a ∥ b, c ∥ d, a ∦ d 
求证: b ∦ c
∵ a ∥ b
   a ∦ d
∴ b ∦ d
∵ c ∥ d
∴ b ∦ c
```

可以用下面的语法来表示：

```
known: parallel(a, b), parallel(c, d), !parallel(a, d)
// 已知 ⇒ 结论
{ parallel(a, b), !parallel(a, d) } ⇒ { !parallel(b, d) }
{ parallel(c, d), !parallel(b, d) } ⇒ { !parallel(b, c) }
```

然后对每一步证明遍历一遍公理和已知然后进行匹配。这样当然很低效，匹配证据的顺序的时间复杂度是指数级的，如果每次手动提供依据就可以大大提高效率，比如改成下面的表示法：

```
// 公理
Axiom parallelAxiom { parallel ( a, b ), !parallel ( a, c ) } ⇒ !parallel ( b, c )
Axiom sym { parallel ( a, b ) } ⇒ parallel ( b, a )
// 证明
parallelogram { p: parallel ( a, b ), 
                q: parallel ( c, d ), 
                r: !parallel ( a, d ) } ⇒ !parallel ( b, c )
    = parallelAxiom ( sym ( q ), sym ( parallelAxiom ( p, r ) ) )
```

细想的话实际上 `parallelogram` 的定义有点像是个函数类型： `p, q, r` 三个依据就像是函数的三个参数，指代的三个命题就像是参数的类型，而证据 `parallelAxiom, sym`
的使用就像是函数调用一样，把一系列已知变换成一个结论。而且 `parallelogram` 这个证明同样也可以作为证据被其他证明使用。

> 注：
>
> 进行暴力匹配找到解的工具中，有一种被称为 SAT Solver

## Curry-Howard 同构

> 命题即类型，证明即程序

Curry-Howard 同构（Curry-Howard Isomorphism, 有些范畴人倾向叫它 Curry-Howard
Correspondence）指出了程序和证明的相似性：一个命题可以看做一个类型，蕴含可以看做函数类型，全称量词可以看做 `forall` ，否定可以看做没有实例的空类型（Empty Type,
Void），析取可以看做和类型，合取可以看做积类型。实际上我们可以按照以上规则将任意证明转化成一段程序，而对程序进行类型检查就是对证明的检查。证明的过程就是利用现有实例构造出指定类型的实例的过程。

利用 Curry-Howard 同构编写的一种类型检查器可以帮助数学家检查证明过程，这样的类型检查器被称为证明辅助器（Proof Assistant）。比较常见的证明辅助器有 Agda, Arend, Coq, Lean, F*
等。一个语言能用作辅助证明，最基本要拥有依赖类型（Dependent Type），例如对于上面的简单证明 `p` 的类型 `parallel ( a, b )` 也会依赖 `a, b` 。不过构造演算的类型系统足够表述上面的证明：

```
parallelAxiom = Axiom (
	(a: Line) → (b: Line) → (c: Line) → 
	parallel a b → !parallel a c → !parallel b c )
sym = Axiom ( 
	(a: Line) → (b: Line) → 
	parallel a b → parallel b a )

parallelogram = 
	(a: Line) ⇒ (b: Line) ⇒ (c: Line) ⇒ (d: Line) ⇒ 
	(p: parallel a b) ⇒ (q: parallel c d) ⇒ (r: !parallel a d) ⇒
	parallelAxiom d b c (sym c d q) (sym b d (parallelAxiom a b d p r))
```

其中 `Axiom` 用于表示公理，公理实际上就是一个包含类型信息的不可计算实例：

```java
class Axiom implements Expr {
    Expr t;
    public Expr reduce() { return this; }
    public Expr fullReduce() { return this; }
    public Expr checkType(Env env) { return t; }
}
```

> 注：
>
> 由于改写顺序，此时还未改写 Rust 的构造演算。

构造出公理时就默认它是正确的，因为我们获得了对应类型的实例。把命题当成公理非常方便但是滥用公理容易造成大问题，如果不慎引入了一个错误的公理那么整个证明都变得不正确了。

> 注：
>
> 个人觉得 `平行` 最好可以定义为一种等价关系，即存在自反对称传递性，然后 `parallelAxiom` 就可以被推导出了。

## 补充：使用 Rust 进行逻辑证明

> 注：
>
> 若读者对一阶命题逻辑有基础理解，对下文将会有更好的理解。
>
> 本节只为了提供 Curry-Howard Correspondence 的一个直观感受。由于本质上 Rust 的类型系统表述力弱，并不能得出此同构。
>
> 并且由于 Rust 的类型系统的完备性存疑（虽然目前存在证明其完备性的尝试），并且标准库内大量存在 unsafe 代码，使用 Rust 进行证明只能作为娱乐项目（
>
> 毕竟，Rust 设计之初就没打算成为证明助手 x x

由于 `min_const_generics` 已经被并入 stable rust，我们实质上已经有了最简陋的依值类型 (dependent type)，具体来说的话我们有了简单的 pi type。 （它的简陋体现在，只能用 int,
bool 等极少量内置类型来作为被依赖的值）

这意味着，我们现在可以在 Rust 的类型系统中表达带全称量词的一阶逻辑。

不幸的是，`never` 类型（换句话说，`bottom`）还在 nightly 阶段，我们仍然无法使用 stable rust 来实现。

下面，我们来尝试证明上文中提到的关于平行的一系列引理：

### 定义命题

我们定义 `平行` 命题（类型）和 `伪` 命题（类型）：

```rust
#[derive(Copy, Clone, Default)]
pub struct Parallel<const a: usize, const b: usize> {}

type False = !;     // never type
```

注意：bottom type (aka. never type, false type) 没有任何构造函数，这意味着我们永远无法证明一个伪命题（构造一个它的表达式）

### 引入公理

首先，我们引入伪命题的相关公理：爆炸原理 (Principle of explosion)。也就是说，如果可以证明伪命题（可以构造底类型的实例），我们可以证明（构造）一切。

```rust
mod Axioms {
    use super::{False, Parallel};

    pub fn bot_elim<T: Default>(contra: False) -> T {
        Default::default()
    }
```

在构造主义逻辑中，我们用`命题推出伪命题（P -> False）`来表达`命题为假 (not P)`这一概念。 显然，我们也不能构造出任何命题（类型） P 的证明（表达式），因为如果可以构造出 P
的证明，那我们也自然可以证明伪命题，那就乱套了。

其次，我们知道 `平行` 是一种等价关系，也就是说它具有自反、对称、传递性：

- 自反性：对于一切直线 A，A ∥ A
- 对称性：对于一切直线 A 和 B，如果 A ∥ B 那么 B ∥ A
- 传递性：对于一切直线 A B 和 C，如果 A ∥ B 并且 B ∥ C 那么 A ∥ C

根据 Curry-Howard Correspondence，蕴含 `->` 表示函数，前提命题是参数的类型，结论命题是返回值的类型。 自然地，这一函数接受前提的证明，提供结论的证明。

> 注：
>
> 另由于 `肯定前件` 以及 `假设引理` 的正确性，我们有柯里同构。柯里同构是指，以下两种函数其实本质是一样的
>
> fn(a: A, b: B) -> C {}
>
> fn(a: A) -> (fn(b: B) -> C) {}
>
> 这两个操作常被分别称为 `柯里化` 与 `逆柯里化`

全称量词意味着，对于所有命题（类型）P，其证明（表达式）都可以被接受，则它对应着 `泛型` 概念。

现在我们来表达平行的等价公理

```rust
    // forall a b, a ∥ b -> b ∥ a
    pub fn sym<const a: usize, const b: usize>(
        p: Parallel<{ a }, { b }>,
    ) -> Parallel<{ b }, { a }> {
        Default::default()
    }

    // forall a b c, a ∥ b -> b ∥ c -> a ∥ c
    pub fn trans<const a: usize, const b: usize, const c: usize>(
        p: Parallel<{ a }, { b }>,
        q: Parallel<{ b }, { c }>,
    ) -> Parallel<{ a }, { c }> {
        Default::default()
    }

    // forall a, a ∥ a
    pub fn refl<const a: usize>() -> Parallel<{ a }, { a }> {
        Default::default()
    }
}

use Axioms::{bot_elim, refl, sym, trans};
```

### 小试牛刀

首先，我们给 `命题为假 (not P)` 提供语法糖

```rust
macro_rules! not {
    ($p: ty) => {
        impl FnOnce($p) -> False
    }
}

macro_rules! not_dyn {
    ($p: ty) => {
        dyn FnOnce($p) -> False
    }
}
```

我们从最简单的东西开始：

矛盾可以推出一切
```rust
// forall P Q, P -> not P -> Q
fn ex_falso<P, Q: Default>(h1: P, contra: not!(P)) -> Q {
    bot_elim(contra(h1))
}
```

来一些具体的矛盾
```rust
// forall a b c, a ∥ b -> a ∦ b -> c ∥ d
fn explosion<const a: usize, const b: usize, const c: usize, const d: usize>(
    h1: Parallel<{ a }, { b }>,
    h2: not!(Parallel<{a}, {b}>),
) -> Parallel<{ c }, { d }> {
    ex_falso(h1, h2)
}
```

接下来，我们证明一些比较有用的事情：

显然地，a ∦ b 推出 b ∦ a
```rust
// forall a b, a ∦ b -> b ∦ a
fn theorem_neg_par_sym<const a: usize, const b: usize>(
    hyp: not!(Parallel<{ a }, { b }>),
    contra: Parallel<{ b }, { a }>,
) -> False {
    hyp(sym(contra))
}
```

> 注：
> 
> 注意到我们这里展开了否定，并使用了逆柯里化。
> 
> 原本我们想证明 `not (a ∥ b) -> not (b ∥ a)`，展开来写就是 `(a ∥ b -> False) -> (b ∥ a -> False)`，
> 由于箭头是右结合的，同时逆柯里化，我们得到 `(not (a ∥ b), b ∥ a) -> False`

### 完成证明

最后，证明最初我们想要的结论：

```
已知: a ∥ b, c ∥ d, a ∦ d
求证: b ∦ c
```

首先我们证明引理：如果 a ∥ b 且 a ∦ c，那么 c ∦ b

```rust
// forall a b c, a ∥ b -> a ∦ c -> c ∦ b
fn lemma<const a: usize, const b: usize, const c: usize>(
    h1: Parallel<{ a }, { b }>,
    h2: not!(Parallel<{ a }, { c }>),
    contra: Parallel<{ c }, { b }>,
) -> False {
    h2(trans(h1, sym(contra)))
}
```

然后，我们即可得到结论

```rust
// forall a b c d, a ∥ b -> c ∥ d -> a ∦ d -> b ∦ c
fn theorem_complex<const a: usize, const b: usize, const c: usize, const d: usize>(
    h1: Parallel<{ a }, { b }>,
    h2: Parallel<{ c }, { d }>,
    h3: not!(Parallel<{ a }, { d }>),
    contra: Parallel<{ b }, { c }>,
) -> False {
    lemma(sym(h2), |contra_| lemma(h1, h3, contra_), contra)
}
```

有了这条定理之后，我们可以将其应用在任何数条直线上，只要其满足前件的要求：

```rust
#[test]
fn reasoning() {
    const A: usize = 1;
    const B: usize = 2;
    const C: usize = 3;
    const D: usize = 4;
    let goal: Box<not_dyn!(Parallel<B, C>)> = box |contra| {
        theorem_complex(
            Parallel::<A, B> {},
            Parallel::<C, D> {},
            |_| loop {}, // this fn never returns
            contra,
        )
    };
}
```

> 注：
>
> 注意到定理的第三个输入参数我们没有给任何类型标注，这是因为 Rust 的类型推导可以自动推出所需的类型（命题）。
> 
> 为了体现出这个函数的返回值（它的类型是 False）不可能构造出来，函数体被填成了一个死循环，意味着它永远不可能被构造出来。
