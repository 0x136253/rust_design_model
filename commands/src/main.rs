//命令模式的基本概念是，将动作分离为单独的对象，并且作为参数传递它们
// 1.利用trait
// 2.利用函数指针，和1本质上等同
// 3.利用Fn trait对象，本质上类似于将函数指针存至Vec中，可以少定义没必要的trait


//如果我们的命令很小，可以定义成函数，或作为闭包传递，那么使用函数指针可能更好， 因为它不需要动态分发。即第2、3种方法
// 但如果我们的命令是个完整的结构， 有一堆函数和变量被分别定义为独立的模块，那么使用trait对象会更合适。 

pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
pub struct AddField;

impl Migration for CreateTable{
    fn execute(&self) -> &str {
       "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}

impl Migration for AddField{
    fn execute(&self) -> &str {
       "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self,cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self)-> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands.iter().rev().map(|cmd| cmd.rollback()).collect()
    }
}
fn main() {
    let mut schema = Schema::new();
    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);


    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

//利用函数指针实现，效果等同

// type FnPtr = fn() -> String;
// struct Command {
//     execute: FnPtr,
//     rollback: FnPtr,
// }

// struct Schema {
//     commands: Vec<Command>,
// }

// impl Schema {
//     fn new() -> Self {
//         Self { commands: vec![] }
//     }
//     fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
//         self.commands.push(Command { execute, rollback });
//     }
//     fn execute(&self) -> Vec<String> {
//         self.commands.iter().map(|cmd| (cmd.execute)()).collect()
//     }
//     fn rollback(&self) -> Vec<String> {
//         self.commands
//             .iter()
//             .rev()
//             .map(|cmd| (cmd.rollback)())
//             .collect()
//     }
// }

// fn add_field() -> String {
//     "add field".to_string()
// }

// fn remove_field() -> String {
//     "remove field".to_string()
// }

// fn main() {
//     let mut schema = Schema::new();
//     schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
//     schema.add_migration(add_field, remove_field);
//     assert_eq!(vec!["create table", "add field"], schema.execute());
//     assert_eq!(vec!["remove field", "drop table"], schema.rollback());
// }


//利用Fn trait对象实现

// type Migration<'a> = Box<dyn Fn() -> &'a str>;

// struct Schema<'a> {
//     executes: Vec<Migration<'a>>,
//     rollbacks: Vec<Migration<'a>>,
// }

// impl<'a> Schema<'a> {
//     fn new() -> Self {
//         Self {
//             executes: vec![],
//             rollbacks: vec![],
//         }
//     }
//     fn add_migration<E, R>(&mut self, execute: E, rollback: R)
//     where
//         E: Fn() -> &'a str + 'static,
//         R: Fn() -> &'a str + 'static,
//     {
//         self.executes.push(Box::new(execute));
//         self.rollbacks.push(Box::new(rollback));
//     }
//     fn execute(&self) -> Vec<&str> {
//         self.executes.iter().map(|cmd| cmd()).collect()
//     }
//     fn rollback(&self) -> Vec<&str> {
//         self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
//     }
// }

// fn add_field() -> &'static str {
//     "add field"
// }

// fn remove_field() -> &'static str {
//     "remove field"
// }

// fn main() {
//     let mut schema = Schema::new();
//     schema.add_migration(|| "create table", || "drop table");
//     schema.add_migration(add_field, remove_field);
//     assert_eq!(vec!["create table", "add field"], schema.execute());
//     assert_eq!(vec!["remove field", "drop table"], schema.rollback());
// }
