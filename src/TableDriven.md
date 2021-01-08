# 十分钟魔法练习：表驱动编程

### By 「玩火」 改写 「光量子」

> 前置技能： 简单 Rust 基础

```rust
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use std::cell::RefCell;
```

## Intro

表驱动编程被称为是普通程序员和高级程序员的分水岭，而它本身并没有那么难，甚至很多时候不知道的人也能常常重新发明它。

而它本身是锻炼抽象思维的良好途径，几乎所有复杂的系统都能利用表驱动法来进行进一步抽象优化，而这也非常考验程序员的水平。

> 注：
>
> 表驱动编程其实是一种非常常见而基础的写法。
> 甚至可以说，只要是写过一段时间程序或者写过某些算法实现，有大概率会在不经意间使用了这种写法。

## 数据表

学编程最开始总会遇到这样的经典习题：

> 输入成绩，返回等第，90 以上 A ，80 以上 B ，70 以上 C ，60 以上 D ，否则为 E

作为一道考察 `if` 语句的习题初学者总是会写出这样的代码：

```rust
fn naive_get_level(score: u64) -> char {
    if score >= 90 { return 'A'; }
    if score >= 80 { return 'B'; }
    if score >= 70 { return 'C'; }
    if score >= 60 { return 'D'; }
    return 'E';
}
```

等学了 `match` 语句以后可以将它改成 `match s/10` 的写法。

但是这两种写法都有个同样的问题：如果需要不断添加等第个数那最终 `(naive_)get_level` 函数就会变得很长很长，最终变得不可维护。

学会循环和数组后回头再看这个程序，会发现这个程序由反复的 `if score >= _ { return _; }` 构成，可以改成循环结构，把对应的数据塞进数组：

```rust
fn get_level(score: u64) -> char {
    const TABLE: [(u64, char); 5] = [
        (100, 'A'),
        (90, 'B'),
        (80, 'C'),
        (70, 'D'),
        (60, 'E')
    ];
    TABLE
        .iter()
        .fold('\0', |current_grade, (max_score, grade)|
            if score <= *max_score { *grade } else { current_grade })
}
```

这样的好处是只需要在两个数组中添加一个值就能加一组等第而不需要碰 `get_level` 的逻辑代码。

而且进一步讲，数组可以被存在外部文件中作为配置文件，与源代码分离，这样不用重新编译就能轻松添加一组等第。

这就是表驱动编程最初阶的形式，通过抽取相似的逻辑并把不同的数据放入表中来避免逻辑重复，提高可读性和可维护性。

再举个带修改的例子，写一个有特定商品的购物车：

```rust
#[derive(Default, Copy, Clone)]
struct Item<'a> {
    pub name: &'a str,
    pub price: u64,
    pub count: u64,
}

#[derive(Clone)]
struct ShopList<'a> {
    items: Vec<Item<'a>>,
}

impl Default for ShopList<'_> {
    fn default() -> Self {
        ShopList {
            items: vec![
                Item {
                    name: "water",
                    price: 1,
                    ..Default::default()
                },
                Item {
                    name: "cola",
                    price: 2,
                    ..Default::default()
                },
                Item {
                    name: "choco",
                    price: 5,
                    ..Default::default()
                },
            ],
        }
    }
}

impl Display for ShopList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.items
                .iter()
                .map(|item| format!("{} (${}/per): {}", item.name, item.price, item.count))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl ShopList<'_> {
    fn buy(&mut self, name: &str) {
        for item in &mut self.items {
            if item.name == name {
                item.count += 1;
            }
        }
    }
}

#[test]
fn test_shop_list() {
    let mut shop_list = ShopList::default();
    shop_list.buy("cola");
    assert_eq!(shop_list.to_string(),
               "water ($1/per): 0\ncola ($2/per): 1\nchoco ($5/per): 0");
}
```

## 逻辑表

初学者在写习题的时候还会碰到另一种没啥规律的东西，比如：

> 用户输入 0 时购买 water ，输入 1 时购买 cola ，输入 2 时打印购买的情况，输入 3 退出系统。

看似没有可以抽取数据的相似逻辑。但是细想一下，真的没有公共逻辑吗？实际上公共的逻辑在于这些都是在同一个用户输入情况下触发的事件，区别就在于不同输入触发的逻辑不一样，那么其实可以就把逻辑制成表：

```rust
struct SimpleUI<'a> {
    shop_list: RefCell<ShopList<'a>>,
    events: Vec<Box<dyn Fn(&Self) -> ()>>,
}

impl Default for SimpleUI<'static> {
    fn default() -> Self {
        SimpleUI {
            shop_list: Default::default(),
            events: vec![
                Box::new(|_self| _self.shop_list.borrow_mut().buy("water")),
                Box::new(|_self| _self.shop_list.borrow_mut().buy("cola")),
                Box::new(|_self| println!("{}", _self.shop_list.borrow())),
            ],
        }
    }
}

impl SimpleUI<'_> {
    fn run_event(&self, e: usize) {
        self.events[e](self)
    }
}
```

这样如果需要添加一个用户输入指令只需要在 `event` 表中添加对应逻辑和索引， 修改用户的指令对应的逻辑也变得非常方便。 这样用户输入和时间触发两个逻辑就不会串在一起，维护起来更加方便。

> 注：
>
> 忍不住想吐槽 Rust 的 borrow checker，由于 events 内会产生对 shop_list 的多个可变引用，
> 不可避免地需要使用 RefCell 来绕掉可变引用编译时排他性的限制，从而产生了内部可变（interior mutability）的行为。
>
> 从本人（改写者）角度来说不是很喜欢这个特性，不过这就是见仁见智的了。

## 自动机

如果再加个逻辑表能修改的跳转状态就构成了自动机（Automaton）。这里举个例子，利用自动机实现了一个复杂的 UI ，在 `menu` 界面可以选择开始玩或者退出，在 `move` 界面可以选择移动或者打印位置或者返回 `menu`
界面：

```rust
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum UIState {
    Menu,
    GamePlay,
}

impl Default for UIState {
    fn default() -> Self {
        UIState::Menu
    }
}

type Jumper = dyn Fn(&ComplexUI, char);
type Draw = dyn Fn(&ComplexUI);

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
struct ComplexUIState {
    state: UIState,
    coord: (i64, i64),
}

struct ComplexUI {
    ui: RefCell<ComplexUIState>,
    jumpers: HashMap<UIState, Box<Jumper>>,
    draw: HashMap<UIState, Box<Draw>>,
}

impl ComplexUI {
    fn jump_to(&self, state: UIState) {
        self.ui.borrow_mut().state = state;
        (self.draw[&state])(self);
    }

    fn run_event(&self, c: char) {
        let state = self.ui.borrow().state;
        (self.jumpers[&state])(self, c);
    }
}

impl Default for ComplexUI {
    fn default() -> Self {
        let menu_jumper = |_self: &ComplexUI, c: char| {
            let mut events: HashMap<char, Box<dyn Fn()>> = HashMap::new();
            events.insert('p', Box::new(|| _self.jump_to(UIState::GamePlay)));
            events.insert('q', Box::new(|| eprintln!("exit")));

            (*events
                .get(&c)
                .unwrap_or(&(Box::new(|| eprintln!("invalid key")) as Box<dyn Fn()>)))(
            );
        };

        let move_jumper = |_self: &ComplexUI, c: char| {
            let mut events: HashMap<char, Box<dyn Fn()>> = HashMap::new();
            events.insert(
                'w',
                Box::new(|| {
                    _self.ui.borrow_mut().coord.1 += 1;
                    _self.draw[&_self.ui.borrow().state](_self);
                }),
            );
            events.insert(
                's',
                Box::new(|| {
                    _self.ui.borrow_mut().coord.1 -= 1;
                    _self.draw[&_self.ui.borrow().state](_self);
                }),
            );
            events.insert(
                'd',
                Box::new(|| {
                    _self.ui.borrow_mut().coord.0 += 1;
                    _self.draw[&_self.ui.borrow().state](_self);
                }),
            );
            events.insert(
                'a',
                Box::new(|| {
                    _self.ui.borrow_mut().coord.0 -= 1;
                    _self.draw[&_self.ui.borrow().state](_self);
                }),
            );
            events.insert('e', Box::new(|| eprintln!("{:?}", _self.ui.borrow().coord)));
            events.insert('q', Box::new(|| _self.jump_to(UIState::Menu)));

            (events
                .get(&c)
                .unwrap_or(&(Box::new(|| eprintln!("invalid key")) as Box<dyn Fn()>)))(
            );
        };

        let mut jumpers: HashMap<UIState, Box<Jumper>> = HashMap::new();
        jumpers.insert(UIState::Menu, Box::new(menu_jumper));
        jumpers.insert(UIState::GamePlay, Box::new(move_jumper));

        let mut draw: HashMap<UIState, Box<Draw>> = HashMap::new();
        draw.insert(
            UIState::Menu,
            Box::new(|_self: &ComplexUI| {
                eprintln!("draw menu");
            }),
        );
        draw.insert(
            UIState::GamePlay,
            Box::new(|_self: &ComplexUI| {
                eprintln!("draw move");
            }),
        );

        ComplexUI {
            ui: Default::default(),
            jumpers,
            draw,
        }
    }
}

#[test]
fn test_ui() {
    let ui = ComplexUI::default();
    ui.run_event('a'); // print: invalid key
    ui.run_event('p'); // jump to gameplay state & draw move
    ui.run_event('e'); // print: (0, 0)
    ui.run_event('w'); // coord changed to (1, 0) & draw move
    ui.run_event('e'); // print: (1, 0)
    ui.run_event('q'); // jump to menu state & draw menu
    ui.run_event('q'); // exit
}
```

> 注 1:
> 
> 这边相较原版来说使用了 enum 来表示状态，用 hashmap 代替了数组。

> 注 2：
> 
> 又是 RefCell 和 interior mutability，还有几处 type checker 推断不出类型导致需要手动标注或者 cast ...
> 
> 建议参考[其他语言的实现](https://github.com/goldimax/magic-in-ten-mins/blob/main/doc/TableDriven.md)，
> 如果有更好的写法欢迎开 Issue 或者 PR