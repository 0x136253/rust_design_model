//临时可变性
//当处理完之后就只会读取而不修改。
//这种情况可以变量重绑定将其改为不可变的。

fn get_vec() ->Vec<i32> {
    vec![1,2,3,4,5]
}

fn main() {
    let data = {
        let mut data = get_vec();
        data.sort();
        data
    };

    //or
    let mut data = get_vec();
    data.sort();
    let data = data;

}