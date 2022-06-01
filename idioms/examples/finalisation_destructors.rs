//使用宏?,来隐式的退出函数
//无论使用宏退出,还是使用return语句,均会执行析构代码

//优:在析构器里的代码退出前总是会被执行，能应对恐慌（panics），提前返回等等。

//缺:不保证析构器里的代码一定会被执行。
//举例来说，函数内有一个死循环或者在退出前函数崩溃的情况。
//在一个已经发生恐慌(panicking)的线程里再次发生恐慌时，析构器也不会执行代码。Rust并不是绝对保证析构器一定会执行
//这种模式介绍了一些难以注意的隐式代码，即函数在结束时没有显式给出析构器执行代码。因此导致debug的时候更加棘手。
//为了确定性，申请一个对象和实现Drop特性增加了很多样板代码。


//注:对象在函数结束前必须保持存活，然后就被销毁。
//这个对象必须是一个值或者独占数据的指针（例如：Box<Foo>）。如果使用一个共享指针（例如Rc）， 
//那么终结器的生命周期就比函数更长了。类似地，终结器不应该被转移所有权到他处或者被返回。

fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    // The dtor of _exit will run however the function `bar` is exited.
    let _exit = Foo;
    // Implicit return with `?` operator.
    baz()?;
    // Normal return.
    Ok(())
}

fn baz() -> Result<(), ()> {
    // Ok(())
    Err(())
}

fn main() {
    match bar() {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err")
    }
}