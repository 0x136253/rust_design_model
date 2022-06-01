use std::ops::Deref;


//使用集合的Deref特性使其像智能指针一样，提供数据的借用或者所有权。
//优:大部分方法可以只针对借用类型实现，这些实现对自有数据的类型可以隐式地适用。 给用户一个获取借用或所有权的选择。
//缺:边界检查时，不考虑仅通过解引用可用的方法和特性，所以对泛型数据结构使用这种模式将会变得复杂。（请看 Borrow和AsRef特性）
struct Vec<T> {
    data: T,
    //..
}

// impl<T> Deref for Vec<T> {
//     type Target = [T];

//     fn deref(&self) -> &[T] {
//         //..
//     }
// }


fn main() {

}