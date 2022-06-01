use std::{mem, fmt::Display};

//避免因Rust借用检查在修改枚举值时使用clone带来的不必要开销
//用mem::{take(_), replace(_)}在修改枚举变体时保持值的所有权
//优:没有内存申请
//缺:有点啰嗦,take操作需要类型实现Default特性。然而，如果这个类型没有实现Default特性，你还是可以用 mem::replace

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

impl Display for MyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyEnum::A{ ref name, x } => write!(f,"MyEnum::A: name->{},x->{}",name,x),
            MyEnum::B{ ref name } => write!(f,"MyEnum::B: name->{}",name),
        }
        
    }
}
enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}


impl Display for MultiVariateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiVariateEnum::A{ ref name } => write!(f,"MultiVariateEnum::A: name->{}",name),
            MultiVariateEnum::B{ ref name } => write!(f,"MultiVariateEnum::B: name->{}",name),
            MultiVariateEnum::C => write!(f,"MultiVariateEnum::C"),
            MultiVariateEnum::D => write!(f,"MultiVariateEnum::D"),
        }
        
    }
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B {
            name: mem::take(name),
        },
        B { ref mut name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}

fn a_to_b(e: &mut MyEnum) {
    // we mutably borrow `e` here. This precludes us from changing it directly
    // as in `*e = ...`, because the borrow checker won't allow it. Therefore
    // the assignment to `e` must be outside the `if let` clause.
    *e = if let MyEnum::A { ref mut name, x: 0 } = *e {
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`, because it is the result of the `if let` expression).
        MyEnum::B {
            name: mem::take(name),
        }

    // In all other cases, we return immediately, thus skipping the assignment
    } else {
        return;
    }
}

fn main() {
    let mut e1 = MyEnum::A{name:"aaa".to_string(),x:0};
    let mut e2 = MyEnum::B{name:"bbb".to_string()};
    a_to_b(&mut e1);
    a_to_b(&mut e2);
    println!("{}", e1);
    println!("{}", e2);

    let mut m1 = MultiVariateEnum::A{name:"aaa".to_string()};
    let mut m2 = MultiVariateEnum::B{name:"bbb".to_string()};
    let mut m3 = MultiVariateEnum::C;
    let mut m4 = MultiVariateEnum::D;
    swizzle(&mut m1);
    swizzle(&mut m2);
    swizzle(&mut m3);
    swizzle(&mut m4);
    println!("{}",m1);
    println!("{}",m2);
    println!("{}",m3);
    println!("{}",m4);
}
