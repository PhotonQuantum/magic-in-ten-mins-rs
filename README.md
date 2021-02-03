# 十分钟魔法练习 (Rust)

改写自 [十分钟魔法练习-玩火](https://github.com/goldimax/magic-in-ten-mins)
原版为 Java 实现

另有
[C++版-图斯卡蓝瑟](https://github.com/tusikalanse/magic-in-ten-mins-cpp) |
[C#版-CWKSC](https://github.com/CWKSC/magic-in-ten-mins-csharp)

抽象与组合

希望能在十分钟内教会你一样魔法

QQ群：1070975853 |
[Telegram Group](https://t.me/joinchat/HZm-VAAFTrIxoxQQ)

> 目录中方括号里的是前置技能。

## 测试所有用例

``` shell script
$ cargo test
```

## 类型系统

[偏易|代数数据类型(Algebraic Data Type)[Rust 基础]](src/ADT.md)

[偏易|广义代数数据类型(Generalized Algebriac Data Type)[Rust 基础，ADT]](src/GADT.md)

[偏易|余代数数据类型(Coalgebraic Data Type)[Rust 基础，ADT]](src/CoData.md)

[偏易|单位半群(Monoid)[Rust 基础]](src/Monoid.md)

[较难|高阶类型(Higher Kinded Type)[Rust 基础]](src/HKT.md)

[中等|单子(Monad)[Rust 基础，HKT]](src/Monad.md)

[较难|状态单子(State Monad)[Rust 基础，HKT，Monad]](src/StateMonad.md)

    [中等|简单类型 λ 演算(Simply-Typed Lambda Calculus)[Java 基础，ADT，λ 演算]](doc/STLC.md)

    [中等|系统 F(System F)[Java 基础，ADT，简单类型 λ 演算]](doc/SystemF.md)

    [中等|系统 Fω(System Fω)[Java 基础，ADT，系统 F]](doc/SysFO.md)

    [较难|构造演算(Calculus of Construction)[Java 基础，ADT，系统 Fω]](doc/CoC.md)

    [偏易|π 类型和 Σ 类型(Pi type & Sigma type)[ADT，构造演算]](doc/PiSigma.md)

## 计算理论

    [较难|λ演算(Lambda Calculus)[Java基础，ADT]](doc/Lambda.md)

    [偏易|求值策略(Evaluation Strategy)[Java基础，λ演算]](doc/EvalStrategy.md)

[较难|丘奇编码(Church Encoding)[λ 演算]](src/ChurchE.md)

    [很难|斯科特编码(Scott Encoding)[构造演算，ADT，μ](doc/ScottE.md)

    [中等|Y 组合子(Y Combinator)[Java 基础，λ 演算，λ 演算编码]](doc/YCombinator.md)

    [中等|μ(Mu)[Java 基础，构造演算， Y 组合子]](doc/Mu.md)

## 编程范式

[简单|表驱动编程(Table-Driven Programming)[简单 Rust 基础]](src/TableDriven.md)

[简单|续延(Continuation)[简单 Rust 基础]](src/Continuation.md)

[中等|代数作用(Algebraic Effect)[简单 Rust 基础，续延]](src/Algeff.md)

    [中等|依赖注入(Dependency Injection)[Java基础，Monad，代数作用]](doc/DepsInj.md)

    [中等|提升(Lifting)[Java基础，HKT，Monad]](doc/Lifting.md)

## 编译原理

    [较难|解析器单子(Parser Monad)[Java基础，HKT，Monad]](doc/ParserM.md)

    [中等|解析器组合子(Parser Combinator)[Java基础，HKT，Monad]](doc/Parsec.md)