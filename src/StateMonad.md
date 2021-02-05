# 十分钟魔法练习：状态单子

### By 「玩火」 改写 「光量子」

> 前置技能：Rust 基础，HKT，Monad

```rust
use crate::HKT::HKT;
use crate::Monad::Monad;
```

## 函数容器

Rust 中的不少容器都是可以看成是单子的，上节中 `List Monad` 的实现就是 Vector 的一层 wrapper，而 `Option Monad` 我们也在标准库中找到了等价物。

不过单子不仅仅可以是实例意义上的容器，也可以是其他抽象意义上的容器，比如函数。

对于一个形如 `S -> A` 形式的函数来说，我们可以把它看成包含了一个 `A` 的惰性容器，只有在给出 `S` 的时候才能知道 `A` 的值。对于这样形式的函数我们同样能写出对应的 `fmap`
，这里就拿状态单子举例子。

## 状态单子

状态单子（State Monad）是一种可以包含一个“可变”状态的单子，尽管状态随着逻辑流在变化，但是在内存里面实际上都是不变量。

其本质就是在每次状态变化的时候将新状态作为代表接下来逻辑的函数的输入。比如对于：

```rust
fn mut_next(mut i: u64) {
    i = i + 1;
    println!("{}", i);
}
```

可以用状态单子的思路改写成：

```rust
fn inmut_next(i: u64) {
    (|i| println!("{}", i))(i + 1);
}
```

State 是一个包含 `run_state` 函数的 Monad，它 (`run_state`) 将某个初态映射到 (终值, 末态)，即 `S -> (A, S)`， 而通过组合可以使变化的状态在逻辑间传递：

```rust
pub struct State<'a, S, A> {
    pub run_state: Box<dyn 'a + Fn(S) -> (A, S)>,
}

impl<'a, S, A> HKT for State<'a, S, A> {
    type Higher<T> = State<'a, S, T>;
}

impl<'a, S: 'a, A: 'a + Clone> Monad<'a, A> for State<'a, S, A> {
    fn pure(v: A) -> Self {
        State {
            run_state: Box::new(move |state: S| (v.clone(), state)),
        }
    }

    fn fmap<F: 'a, B>(self, f: F) -> Self::Higher<B>
        where
            F: Fn(A) -> Self::Higher<B>,
    {
        State {
            run_state: Box::new(move |state: S| {
                let (interm_value, interm_state) = (self.run_state)(state);
                (f(interm_value).run_state)(interm_state)
            }),
        }
    }
}
```

`pure` 操作直接返回当前状态和给定的值， `flatMap` 操作只需要把 `ma` 中的 `A` 取出来然后传给 `f` ，并处理好 `state` 。

> 注
>
> `flatMap` 其实是将两个 State 进行组合，前一个 State 的终值成为了 f 的参数得到一个新的 State，
> 然后向新的 State 输入前一 State 的终态可以得到组合后 State 的终值和终态。

仅仅这样的话 `State` 使用起来并不方便，还需要定义一些常用的操作来读取写入状态：

```rust
pub fn get<'a, S: Clone>() -> State<'a, S, S> {
    State { run_state: Box::new(|state: S| (state.clone(), state)) }
}

pub fn put<'a, S: 'a + Clone>(state: S) -> State<'a, S, ()> {
    State { run_state: Box::new(move |_| ((), state.clone())) }
}

pub fn modify<'a, S: 'a + Clone>(f: impl Fn(S) -> S + 'a) -> State<'a, S, ()> {
    State::fmap(
        get(),
        move |x| put(f(x))
    )
}

impl<'a, S: 'a + Clone, A: 'a> State<'a, S, A> {
    pub fn run(self, state: S) -> (A, S) {
        (self.run_state)(state)
    }

    pub fn eval(self, state: S) -> A {
        (self.run_state)(state).0
    }
}
```

## 使用例

求斐波那契数列：

```rust
fn fib(n: u64) -> State<'static, (u64, u64), u64> {
    match n {
        0 => State::fmap(
            get(),
            |x: (u64, u64)| { State::pure(x.0) }
        ),
        _ => State::fmap(
            modify(|x: (u64, u64)| { (x.1, x.0 + x.1) }),
            move |_| fib(n - 1)
        )
    }
}
```

`fib` 函数对应的 Haskell 代码是：

```haskell
fib :: Int -> State (Int, Int) Int
fib 0 = do
  (_, x) <- get
  pure x
fib n = do
  modify (\(a, b) -> (b, a + b))
  fib (n - 1)
```

~~看上去简单很多~~

> 注：
>
> Rust 版看上去似乎差不多，而且可以通过 macro 改造成 do notation

```rust
fn fib_with_do(n: u64) -> State<'static, (u64, u64), u64> {
    match n {
        0 => mdo!(State<_, _>,
            (x, _) <- get();
            pure x
        ),
        _ => mdo!(State<_, _>,
            modify(|x: (u64, u64)|(x.1, x.0+x.1));
            fib_with_do(n-1)
        ),
    }
}
```

## 有什么用

求斐波那契数列有着更简单的写法：

```rust
fn imp_fib(n: usize) -> u64 {
    let mut a: [u64; 3] = [0, 1, 1];
    for i in 0..(n - 1) {
        a[(i + 2) % 3] = a[(i + 1) % 3] + a[i % 3];
    }
    a[n % 3]
}
```

两种实现的区别体现在：

- 使用了可变对象，而 `State Monad` 仅使用了不可变对象，使得函数是纯函数，但又存储了变化的状态。

- 非递归，如果改写成递归形式需要在 `fib` 上加一个状态参数，`State Monad` 则已经携带。

- `State Monad` 的实现是 **可组合** 的，即可以将任意两个状态类型相同的 `State Monad` 组合起来。

> 注：
>
> 当然还有更 naive 的，无需可变对象，无需传递状态，纯函数的写法，但是其时间复杂度不是线性的

```rust
fn naive_fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => naive_fib(n - 1) + naive_fib(n - 2),
    }
}
```

对应的 Haskell 代码是

```haskell
fib 0 = 0
fib 1 = 1
fib n = fib (n-1) + fib (n-2)
```

```rust
#[test]
fn test_fib() {
    assert_eq!(fib(7).eval((0, 1)), 13);
    assert_eq!(fib_with_do(7).eval((0, 1)), 13);
    assert_eq!(imp_fib(7), 13);
    assert_eq!(naive_fib(7), 13);
}
```