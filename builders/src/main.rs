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