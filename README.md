# Rust设计模式

### You Aren't Going to Need It.(YAGNI原则)

### The best code I ever wrote was code I never wrote.

###  (设计模式如果滥用，那将会增加程序不必要的复杂性)

- **设计模式** 是编写软件过程中解决常见问题的方法。
- **反模式** 是解决常见问题的方法。 然而设计模式带给我们好处，反模式却带来更多的问题

#### 一.设计模式

创建型模式，共五种：工厂方法模式、抽象工厂模式、单例模式、建造者模式、原型模式。

结构型模式，共七种：适配器模式、装饰器模式、代理模式、外观模式、桥接模式、组合模式、享元模式。

行为型模式，共十一种：策略模式、模板方法模式、观察者模式、迭代子模式、责任链模式、命令模式、备忘录模式、状态模式、访问者模式、中介者模式、解释器模式。

#### 1.工厂方法模式(factory-method)

```rust
/**
 * 工厂类：
 * 可以把不同的子类对象，放到一个貌似通用工厂类中。
 * 用起来方便、统一。
 */

/// 工厂类的抽象方法
trait Shape {
    fn areas(&self) -> f64;
}

/// 可以创建多个子类，就象工厂有多个产品一样
enum ShapeType {
    Circle,
    Triangle
}

//圆形子类定义
struct Circle {
    radius:f64
}
//圆形子类实现
impl Shape for Circle {
    fn areas(&self) ->f64{
        self.radius * self.radius * 3.14159
    }
}
//三角形子类定义
struct Triangle {
    buttom:f64,
    height:f64
}
//为三角形子类实现工厂方法
impl Shape for Triangle {
    fn areas(&self) ->f64{
        self.buttom * self.height * 0.5
    }
}

struct ShapeFactory;
impl ShapeFactory {
    fn new_shape(s: &ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle => Box::new(Circle {radius:4.0}),
            ShapeType::Triangle => Box::new(Triangle {buttom:5.0,height:3.0}),
        }
    }
}



fn main() {
    let shape = ShapeFactory::new_shape(&ShapeType::Circle);
    let a= shape.areas(); 
    println!("Circle areas is: {}", a);

    let shape = ShapeFactory::new_shape(&ShapeType::Triangle);
    let a= shape.areas();
    println!("Triangle areas is: {}", a);
}
```



#### 2.抽象工厂模式(abstract-factory)

```rust

pub trait Button {
    fn press(&self);
}

pub trait Checkbox {
    fn switch(&self);
}

///  抽象工厂是通过泛型实现的，它允许编译器创建一个不需要在运行时进行动态调度的代码。
pub trait GuiFactory {
    type B: Button;
    type C: Checkbox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}

/// 使用Box指针定义的抽象工厂。
pub trait GuiFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}
```



#### 3.单例模式(Singleton)

**虽可实现，但不推荐**

```rust
//! 在Rust中实现Singleton的一个纯粹安全的方法是不使用静态变量
//! 并通过函数参数传递一切。
//! 最古老的活体变量是在`main()`开始时创建的一个对象。

fn change(global_state: &mut u32) {
    *global_state += 1;
}

fn main() {
    let mut global_state = 0u32;

    change(&mut global_state);

    println! ("最终状态: {}", global_state);
}	
```



```rust
//! 从Rust 1.63开始，使用全局可变的单例
//虽然在大多数情况下，避免使用全局变量仍然是最好的选择。
//！情况下，还是要避免使用全局变量。
//!
//! 现在`Mutex::new`是`const`，你可以使用全局静态的`Mutex`锁了
//! 而不需要懒惰的初始化。

use std::sync::Mutex。

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new() )。

fn do_a_call() {
    ARRAY.lock().unwrap().push(1)。
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println! ("Called {} times", ARRAY.lock().unwrap().len() );
}
```



```rust
//! Rust并不允许没有`unsafe`的单子模式，因为它
//! 没有一个安全的可变全局状态。
//!
//! `lazy-static`允许声明一个静态变量，并在第一次访问时进行懒惰初始化
//! 第一次访问时。它实际上是通过`unsafe`与`static mut`实现的。
//! 操作，然而，它使你的代码没有`不安全`块。
//!
//! `Mutex`提供对单个对象的安全访问。

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec! );
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1)。
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println! ("Called {}", ARRAY.lock().unwrap().len() );
}	
```



#### 4.建造者模式(Builder)

通过调用建造者来构造对象。

```rust
//建造者模式，看代码，一目了然
//由于Rust中缺少重载，该模式在其中比较常见，但不是都符合T和TBuilder的命名模式，e.g. std::process::Command 是 Child的构造
//优:将构造方法与其他方法分开。防止构造器数量过多。即使构造器本身很复杂，也可以做到封装后一行初始化。
//缺:与直接构造一个结构体或者一个简单的构造函数相比，这种方法太复杂。

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

impl Foo {
    // 入口函数
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new(/* ... */) -> FooBuilder {
        // 设置Foo的最低要求字段
        FooBuilder {
            bar: String::from("X"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        // 在建造者上设置bar，并按值返回。
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        // 构建
        Foo { bar: self.bar }
    }
}

#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();
    assert_eq!(foo, foo_from_builder);
}


fn main() {
    
}
```



#### 5.原型模式(Prototype)

就是`Clone`,也可以通过实现`From`来实现,略过



结构型模式，共七种：适配器模式、装饰器模式、代理模式、外观模式、桥接模式、组合模式、享元模式。

#### 1.适配器模式(Adapter)

```rust
//有一个call函数只接受接口 （trait）为Target的参数：
fn call(target: impl Target);

pub trait Target {
    fn request(&self) -> String;
}

//call函数可以接受的
pub struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        "Ordinary request.".into()
    }
}

//call函数不可以接受的
pub struct SpecificTarget;

impl SpecificTarget {
    pub fn specific_request(&self) -> String {
        ".tseuqer cificepS".into()
    }
}


///适配转换器将SpecificTarget转换为Target
///将adaptee的'SpecificTarget'转换为兼容的`Target`输出
pub struct TargetAdapter {
    adaptee: SpecificTarget,
}

impl TargetAdapter {
    pub fn new(adaptee: SpecificTarget) -> Self {
        Self { adaptee }
    }
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
       // 这里是将原来的specific输出 "改编 "为兼容Target的输出。
        self.adaptee.specific_request().chars().rev().collect()
    }
}
```



#### 2.装饰器模式(Decorator)

```rust
//利用派生宏实现
//#[derive(SomeName)]

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

//Rust的宏还在编译时起作用，而不是像其他语言Java或Python在运行时起作用，更接近于装饰器模式
```



#### 3.代理模式(Proxy)

在某些情况下，一个客户类不想或者不能直接引用一个委托对象，而代理类对象可以在客户类和委托对象之间起到中介的作用，其特征是代理类和委托类实现相同的接口。

```rust
pub trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
}

pub struct Application;

impl Server for Application {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
        if url == "/app/status" && method == "GET" {
            return (200, "Ok".into());
        }

        if url == "/create/user" && method == "POST" {
            return (201, "User Created".into());
        }

        (404, "Not Ok".into())
    }
}

/// NGINX server 代理
pub struct NginxServer {
    application: Application,
    max_allowed_requests: u32,
    rate_limiter: HashMap<String, u32>,
}

impl NginxServer {
    pub fn new() -> Self {
        Self {
            application: Application,
            max_allowed_requests: 2,
            rate_limiter: HashMap::default(),
        }
    }

    pub fn check_rate_limiting(&mut self, url: &str) -> bool {
        let rate = self.rate_limiter.entry(url.to_string()).or_insert(1);

        if *rate > self.max_allowed_requests {
            return false;
        }

        *rate += 1;
        true
    }
}

impl Server for NginxServer {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
        if !self.check_rate_limiting(url) {
            return (403, "Not Allowed".into());
        }

        self.application.handle_request(url, method)
    }
}
```



#### 4.外观模式(Facade)

该模式就是把一些复杂的流程封装成一个接口供给外部用户更简单的使用

API其实是一种外观模式的实现，在API背后隐藏了一个复杂的逻辑。

外观模式类似于组合模式，都是将复杂或繁杂的细节隐藏起来，提供一个干净的外观门面

```rust
use crate::{
    account::Account, ledger::Ledger, notification::Notification, security_code::SecurityCode,
    wallet::Wallet,
};

/// Facade hides a complex logic behind the API.
pub struct WalletFacade {
    account: Account,
    wallet: Wallet,
    code: SecurityCode,
    notification: Notification,
    ledger: Ledger,
}

impl WalletFacade {
    pub fn new(account_id: String, code: u32) -> Self {
        println!("Starting create account");

        let this = Self {
            account: Account::new(account_id),
            wallet: Wallet::new(),
            code: SecurityCode::new(code),
            notification: Notification,
            ledger: Ledger,
        };

        println!("Account created");
        this
    }

    pub fn add_money_to_wallet(
        &mut self,
        account_id: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting add money to wallet");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.credit_balance(amount);
        self.notification.send_wallet_credit_notification();
        self.ledger.make_entry(account_id, "credit".into(), amount);
        Ok(())
    }

    pub fn deduct_money_from_wallet(
        &mut self,
        account_id: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting debit money from wallet");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.debit_balance(amount);
        self.notification.send_wallet_debit_notification();
        self.ledger.make_entry(account_id, "debit".into(), amount);
        Ok(())
    }
}
```



#### 5.桥接模式(Bridge)

定义： 将业务逻辑或巨大的类型划分为独立的类型层次，可以独立开发

案例，遥控器能遥控电视或收音机等设备，将遥控器与被遥控操作的设备分离，由于有共同的接口，相同的遥控器可以与不同的设备一起工作，估计是通过红外，这样的遥控器称为万能遥控器。



**设备代码:**

```rust
pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn volume(&self) -> u8;
    fn set_volume(&mut self, percent: u8);
    fn channel(&self) -> u16;
    fn set_channel(&mut self, channel: u16);
    fn print_status(&self);
}
```



**设备具体实现电视机：**

```rust
use super::Device;

#[derive(Clone)]
pub struct Tv {
    on: bool,
    volume: u8,
    channel: u16,
}

impl Default for Tv {
    fn default() -> Self {
        Self {
            on: false,
            volume: 30,
            channel: 1,
        }
    }
}

impl Device for Tv {
    fn is_enabled(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
    }

    fn disable(&mut self) {
        self.on = false;
    }

    fn volume(&self) -> u8 {
        self.volume
    }

    fn set_volume(&mut self, percent: u8) {
        self.volume = std::cmp::min(percent, 100);
    }

    fn channel(&self) -> u16 {
        self.channel
    }

    fn set_channel(&mut self, channel: u16) {
        self.channel = channel;
    }

    fn print_status(&self) {
        println!("------------------------------------");
        println!("| I'm TV set.");
        println!("| I'm {}", if self.on { "enabled" } else { "disabled" });
        println!("| Current volume is {}%", self.volume);
        println!("| Current channel is {}", self.channel);
        println!("------------------------------------\n");
    }
}
```



**收音机:**

```rust
use super::Device;

#[derive(Clone)]
pub struct Radio {
    on: bool,
    volume: u8,
    channel: u16,
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            on: false,
            volume: 30,
            channel: 1,
        }
    }
}

impl Device for Radio {
    fn is_enabled(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
    }

    fn disable(&mut self) {
        self.on = false;
    }

    fn volume(&self) -> u8 {
        self.volume
    }

    fn set_volume(&mut self, percent: u8) {
        self.volume = std::cmp::min(percent, 100);
    }

    fn channel(&self) -> u16 {
        self.channel
    }

    fn set_channel(&mut self, channel: u16) {
        self.channel = channel;
    }

    fn print_status(&self) {
        println!("------------------------------------");
        println!("| I'm radio.");
        println!("| I'm {}", if self.on { "enabled" } else { "disabled" });
        println!("| Current volume is {}%", self.volume);
        println!("| Current channel is {}", self.channel);
        println!("------------------------------------\n");
    }
}
```



**遥控器:**

```rust
mod advanced;
mod basic;

pub use advanced::AdvancedRemote;
pub use basic::BasicRemote;

use crate::device::Device;

pub trait HasMutableDevice<D: Device> {
    fn device(&mut self) -> &mut D;
}

pub trait Remote<D: Device>: HasMutableDevice<D> {
    fn power(&mut self) {
        println!("Remote: power toggle");
        if self.device().is_enabled() {
            self.device().disable();
        } else {
            self.device().enable();
        }
    }

    fn volume_down(&mut self) {
        println!("Remote: volume down");
        let volume = self.device().volume();
        self.device().set_volume(volume - 10);
    }

    fn volume_up(&mut self) {
        println!("Remote: volume up");
        let volume = self.device().volume();
        self.device().set_volume(volume + 10);
    }

    fn channel_down(&mut self) {
        println!("Remote: channel down");
        let channel = self.device().channel();
        self.device().set_channel(channel - 1);
    }

    fn channel_up(&mut self) {
        println!("Remote: channel up");
        let channel = self.device().channel();
        self.device().set_channel(channel + 1);
    }
}

pub struct BasicRemote<D: Device> {
    device: D,
}

impl<D: Device> BasicRemote<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }
}

impl<D: Device> HasMutableDevice<D> for BasicRemote<D> {
    fn device(&mut self) -> &mut D {
        &mut self.device
    }
}

impl<D: Device> Remote<D> for BasicRemote<D> {}
```



#### 6.组合模式(Composite)

定义：有时又叫作部分-整体模式，它是一种将对象组合成树状的层次结构的模式，用来表示“部分-整体”的关系，使用户对单个对象和组合对象具有一致的访问性。

意图：将对象组合成树形结构以表示"部分-整体"的层次结构。组合模式使得用户对单个对象和组合对象的使用具有一致性。

```rust
pub trait Component {
    fn search(&self, keyword: &str);
}

//File
pub struct File {
    name: &'static str,
}

impl File {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Component for File {
    fn search(&self, keyword: &str) {
        println!("Searching for keyword {} in file {}", keyword, self.name);
    }
}

//Folder
pub struct Folder {
    name: &'static str,
    components: Vec<Box<dyn Component>>,
}

impl Folder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            components: vec![],
        }
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}

impl Component for Folder {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in folder {}",
            keyword, self.name
        );

        for component in self.components.iter() {
            component.search(keyword);
        }
    }
}


fn main() {
    let file1 = File::new("File 1");
    let file2 = File::new("File 2");
    let file3 = File::new("File 3");

    let mut folder1 = Folder::new("Folder 1");
    folder1.add(file1);

    let mut folder2 = Folder::new("Folder 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    folder2.search("rose");
}
```



#### 7.享元模式(Flyweight)

定义：通过共享的方式高效的支持大量细粒度的对象。

主要解决：在有大量对象时，有可能会造成内存溢出，我们把其中共同的部分抽象出来，如果有相同的业务请求，直接返回在内存中已有的对象，避免重新创建。

```rust
pub enum TreeColor {
    Color1,
    Color2,
    TrunkColor,
}

pub struct TreeKind {
    color: TreeColor,
    _name: String,
    _data: String,
}

pub struct Tree {
    x: u32,
    y: u32,
    kind: Rc<TreeKind>,
}


pub struct Forest {
    cache: HashSet<Rc<TreeKind>>,
    trees: Vec<Tree>,
}
```





行为型模式，共十一种

| 父类与子类   | 两个类之间 | 类的状态   | 通过中间类 |
| ------------ | ---------- | ---------- | ---------- |
| 策略模式     | 观察者模式 | 备忘录模式 | 访问者     |
| 模板方法模式 | 迭代器模式 | 状态模式   | 中介者模式 |
|              | 责任链模式 |            | 解释器模式 |
|              | 命令模式   |            |            |

#### 1.策略模式(Strategy)

定义： 策略模式定义了一系列算法，并将每个算法封装起来，使他们可以相互替换，且算法的变化不会影响到使用算法的客户。

```rust
trait Strategy {
    fn execute(&self);
}

//策略A
struct ConcreteStrategyA;
impl Strategy for ConcreteStrategyA {
    fn execute(&self) {
        println!("ConcreteStrategyA")
    }
}

//策略B
struct ConcreteStrategyB;
impl Strategy for ConcreteStrategyB {
    fn execute(&self) {
        println!("ConcreteStrategyB")
    }
}


struct Context<S> {
    strategy: S,
}
impl<S> Context<S>
where
    S: Strategy,
{
    fn do_things(&self) {
        println!("Common preamble");
        self.strategy.execute();
        println!("Common postamble");
    }
}
```



#### 2.模板方法模式(template-method)

定义：定义一个操作中算法的骨架，而将一些步骤延迟到子类中，模板方法使得子类可以不改变算法的结构即可重定义该算法的某些特定步骤。

```rust
trait TemplateMethod {
    fn template_method(&self) {
        self.base_operation1();
        self.required_operations1();
        self.base_operation2();
        self.hook1();
        self.required_operations2();
        self.base_operation3();
        self.hook2();
    }

    fn base_operation1(&self) {
        println!("TemplateMethod says: I am doing the bulk of the work");
    }

    fn base_operation2(&self) {
        println!("TemplateMethod says: But I let subclasses override some operations");
    }

    fn base_operation3(&self) {
        println!("TemplateMethod says: But I am doing the bulk of the work anyway");
    }

    fn hook1(&self) {}
    fn hook2(&self) {}

    fn required_operations1(&self);
    fn required_operations2(&self);
}

struct ConcreteStruct1;

impl TemplateMethod for ConcreteStruct1 {
    fn required_operations1(&self) {
        println!("ConcreteStruct1 says: Implemented Operation1")
    }

    fn required_operations2(&self) {
        println!("ConcreteStruct1 says: Implemented Operation2")
    }
}

struct ConcreteStruct2;

impl TemplateMethod for ConcreteStruct2 {
    fn required_operations1(&self) {
        println!("ConcreteStruct2 says: Implemented Operation1")
    }

    fn required_operations2(&self) {
        println!("ConcreteStruct2 says: Implemented Operation2")
    }
}

fn client_code(concrete: impl TemplateMethod) {
    concrete.template_method()
}

fn main() {
    println!("同一客户代码可以与不同的具体实现一起工作:");
    client_code(ConcreteStruct1);
    println!();

    println!("同一客户代码可以与不同的具体实现一起工作:");
    client_code(ConcreteStruct2);
}
```



#### 3.观察者模式(Observer)

观察者是一种行为设计模式，它允许一些对象通知其他对象其状态的变化。

```rust
//编译期无法确定大小，解决方案:
//1.使用callback函数，不保存对象，保存类似函数指针,没有生命周期问题或类型擦除
//2.lambda 函数，与2类似，用函数而非对象作为订阅者
//3.结合泛型实现，原理为通过遍历泛型对象，从而在编译期确定对象大小
//impl<'a, T: IObserver + PartialEq> ISubject<'a, T> for Observable<'a, T>
impl Observer for A {
    fn event(&mut self, ev: &String) {
        println!("Got event from observable: {}", ev);
    }
}

struct Observable {
    observers: Vec<dyn Observer>, //如何保存观察者引用，ERROR:无法在编译期确定observers大小
}

impl Observable {
    fn new() -> Observable {
        Observable {
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, o: &dyn Observer) {
        // ERROR
        self.observers.push(o);
    }

    fn remove_observer(&mut self, o: &dyn Observer) {
        // ERROR
        self.observers.remove(o);
    }

    fn notify_observers(&self, ev: &String) {
        for o in &mut self.observers {
            o.event(ev);
        }
    }
}

//在垃圾收集的语言中，通常会Observable引用Observer（通知它）和Observer引用Observable（注销自身）......
//这会在所有权方面造成一些挑战（谁比谁活得长？），这就需要完全整体“取消注册通知”。
//解决方案:使用弱引用解决循环依赖，Weak<RefCell<dyn Observer>>
```



#### 4.迭代器模式(Iterator)

顺序遍历复杂的数据结构而不暴露其内部细节

实现`Iterator`即可

```rust
impl Iterator for UserIterator<'_> {
    fn next(&mut self) -> Option<Self::Item>;
}

let users = UserCollection::new();
let mut iterator = users.iter();

iterator.next();
```



#### 5.责任链模式(chain-of-responsibility)



#### 6.命令模式(Command)

命令模式的基本概念是，将动作分离为单独的对象，并且作为参数传递它们

定义两个数据库操作，`建表`和`加字段`。每个操作都是一个命令

```rust
//命令模式的基本概念是，将动作分离为单独的对象，并且作为参数传递它们
// 1.利用trait
// 2.利用函数指针，和1本质上等同
// 3.利用Fn trait对象，本质上类似于将函数指针存至Vec中，可以少定义没必要的trait


//如果我们的命令很小，可以定义成函数，或作为闭包传递，那么使用函数指针可能更好， 因为它不需要动态分发。即第2、3种方法
// 但如果我们的命令是个完整的结构， 有一堆函数和变量被分别定义为独立的模块，那么使用trait对象会更合适。 

pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
pub struct AddField;

impl Migration for CreateTable{
    fn execute(&self) -> &str {
       "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}

impl Migration for AddField{
    fn execute(&self) -> &str {
       "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self,cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self)-> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands.iter().rev().map(|cmd| cmd.rollback()).collect()
    }
}
fn main() {
    let mut schema = Schema::new();
    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);


    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

//利用函数指针实现，效果等同

// type FnPtr = fn() -> String;
// struct Command {
//     execute: FnPtr,
//     rollback: FnPtr,
// }

// struct Schema {
//     commands: Vec<Command>,
// }

// impl Schema {
//     fn new() -> Self {
//         Self { commands: vec![] }
//     }
//     fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
//         self.commands.push(Command { execute, rollback });
//     }
//     fn execute(&self) -> Vec<String> {
//         self.commands.iter().map(|cmd| (cmd.execute)()).collect()
//     }
//     fn rollback(&self) -> Vec<String> {
//         self.commands
//             .iter()
//             .rev()
//             .map(|cmd| (cmd.rollback)())
//             .collect()
//     }
// }

// fn add_field() -> String {
//     "add field".to_string()
// }

// fn remove_field() -> String {
//     "remove field".to_string()
// }

// fn main() {
//     let mut schema = Schema::new();
//     schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
//     schema.add_migration(add_field, remove_field);
//     assert_eq!(vec!["create table", "add field"], schema.execute());
//     assert_eq!(vec!["remove field", "drop table"], schema.rollback());
// }


//利用Fn trait对象实现

// type Migration<'a> = Box<dyn Fn() -> &'a str>;

// struct Schema<'a> {
//     executes: Vec<Migration<'a>>,
//     rollbacks: Vec<Migration<'a>>,
// }

// impl<'a> Schema<'a> {
//     fn new() -> Self {
//         Self {
//             executes: vec![],
//             rollbacks: vec![],
//         }
//     }
//     fn add_migration<E, R>(&mut self, execute: E, rollback: R)
//     where
//         E: Fn() -> &'a str + 'static,
//         R: Fn() -> &'a str + 'static,
//     {
//         self.executes.push(Box::new(execute));
//         self.rollbacks.push(Box::new(rollback));
//     }
//     fn execute(&self) -> Vec<&str> {
//         self.executes.iter().map(|cmd| cmd()).collect()
//     }
//     fn rollback(&self) -> Vec<&str> {
//         self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
//     }
// }

// fn add_field() -> &'static str {
//     "add field"
// }

// fn remove_field() -> &'static str {
//     "remove field"
// }

// fn main() {
//     let mut schema = Schema::new();
//     schema.add_migration(|| "create table", || "drop table");
//     schema.add_migration(add_field, remove_field);
//     assert_eq!(vec!["create table", "add field"], schema.execute());
//     assert_eq!(vec!["remove field", "drop table"], schema.rollback());
// }

```

#### 7.备忘录模式(Memento)

#### 8.状态模式(State)

#### 9.访问者模式(Vistor)

#### 10.中介者模式(Mediator)

#### 11.解释器模式(Interpreter)



#### 二.反模式

#### 1.Clone过借用检查

```rust
// 定义任意变量
let mut x = 5;

// 借用 `x`（先clone）
let y = &mut (x.clone());

// 由于 x.clone(), x 并未被借用, 这行代码可以运行。
println!("{}", x);

// 用这个借用做点什么，防止因Rust优化直接砍掉这个借用
*y += 1;
```

使用`.clone()`会导致数据被复制。两者之间的任何变化都不会同步——因为会有两个完全独立的变量存在。(Rc<T>和Arc<T>除外)

#### 2.\#![deny(warnings)]

通过禁用编译器生成警告，放弃Rust稳定性。

#### 3.`Deref` 多态

滥用`Deref`特性，模拟结构体之间的继承，从而重用方法。

```rust
use std::ops::Deref;

struct Foo {}

impl Foo {
    fn m(&self) {
        //..
    }

}

struct Bar {
    f: Foo,
}

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Foo {
        &self.f
    }
}

fn main() {
    let b = Bar { f: Foo {} };
    b.m();
}
```

Rust中没有结构体的继承。取而代之的是我们使用组合方式在`Bar`内包含`Foo`

为`Bar`实现了`Deref`特性，生成目标为`Foo`（返回的是内置的`Foo`字段）。这就相当于对`Bar`解引用的时候就会获得到一个`Foo`对象。
