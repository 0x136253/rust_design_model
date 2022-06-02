fn main() {
    use std::rc::Rc;
    //闭包定义的时候就把哪些是复制的数据搞清楚，这样结束时无论闭包有没有消耗掉这些值，都会及早drop掉。
    //闭包能用与上下文相同的变量名来用那些复制或者move进来的变量。
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);
    let closure = {
        // `num1` is moved
        let num2 = num2.clone(); // `num2` is cloned
        let num3 = num3.as_ref(); // `num3` is borrowed
        move || {
            *num1 + *num2 + *num3;
        }
    };

    //将所有变量全部move进去
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);
    let num2_cloned = num2.clone();
    let num3_borrowed = num3.as_ref();
    let closure = move || {
        *num1 + *num2_cloned + *num3_borrowed;
    };
}
