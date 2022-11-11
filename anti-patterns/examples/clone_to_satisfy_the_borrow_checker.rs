fn main() {
    // 定义任意变量
    let mut x = 5;

    // 借用 `x`（先clone）
    let y = &mut (x.clone());

    // 由于 x.clone(), x 并未被借用, 这行代码可以运行。
    println!("{}", x);

    // 用这个借用做点什么，防止因Rust优化直接砍掉这个借用
    *y += 1;

    println!("{}",y);
    println!("{}", x);
}
