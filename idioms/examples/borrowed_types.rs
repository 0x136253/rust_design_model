use std::fmt::Display;



// 当你为函数选择参数类型时，使用带强制隐式转换的目标会增加你代码的复杂度。
// 在这种情况下，函数将会接受更多的输入参数类型。使用可切片类型或者胖指针类型没有限制。
// 事实上，你应该总是用借用类型（borrowed type）, 而不是自有数据类型的借用（borrowing the owned type）。
//  例如&str 而非 &String, &[T] 而非 &Vec<T>, 或者 &T 而非 &Box<T>.

// 当自有数据结构（owned type）的实例已经提供了一个访问数据的间接层时，使用借用类型可以让你避免增加间接层。
// 举例来说，String类型有一层间接层，所以&String将有两个间接层。我们可以用&Str来避免这种情况，无论何时调用函数，强制&String转换为&Str。

fn three_vowels(word: &str) -> bool {  //&str 也可替换为&String,但是会导致调用参数为&str时报错
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

fn print_vec<T:Display>(v:&[T]){ //同上，使用&[T],而非&Vec<T>,避免使用中间层
    for i in v.iter() {
        print!("{} ",i);
    }
    println!();

}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // 至此运行正常，但下面两行就会失败:
    println!("Ferris: {}", three_vowels("Ferris"));
    println!("Curious: {}", three_vowels("Curious"));

    let v1 = [1,2,3,4,5];
    let v2:Vec<i32> = vec![6,7,8,9,0];

    print_vec(&v1);
    print_vec(&v2);
}
