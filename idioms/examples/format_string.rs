// 优:使用format! 连接字符串通常更加简洁和易于阅读
// 缺:它通常不是最有效的连接字符串的方法。
// 对一个可变的String类型对象进行一连串的push操作通常是最有效率的（尤其这个字符串已经预先分配了足够的空间）
fn say_hello(name: &str) -> String {
    // 我们可以手动构建字符串
    // let mut result = "Hello ".to_owned();
    // result.push_str(name);
    // result.push('!');
    // result

    // 但是用format! 更好
    format!("Hello {}!", name)
}

fn main() {
    println!("{}",say_hello("klice"));
}
