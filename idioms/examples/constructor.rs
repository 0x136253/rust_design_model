use std::fmt::Display;

// Rust 没有语言层面的构造器。 取而代之的是常用一个[关联函数][] new 创建对象：
// Rust 也支持默认的Default trait 构造器
// 若struct中所有成员均实现了Default trait，也可用宏#[derive(Default)]实现
// Default特性也为其余容器和泛型提供支持 e.g. Option::unwrap_or_default()
#[derive(Default)]
struct Second{
    value:u64,
}

impl Second {
    // Constructs a new instance of [`Second`].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

// impl Default for Second {
//     fn default() -> Self {
//         Self { value: 0 }
//     }
// }

impl Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.value)
    }
}

fn main() {
    let x = Second::new(12);
    let y = Second::default();
    println!("new:{},Default:{}",x,y)
}