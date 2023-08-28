use std::collections::{hash_map, HashMap};

macro_rules! fn_rules_println {
    () => {
        println!("这是宏表达式")
    };
}

#[derive(Debug)]
struct S;

//创建一个函数
macro_rules! create_func {
    ($idt:ident) => {
        fn $idt() {
            println!("这是宏创建了一个函数")
        }
    };
}
macro_rules! test {
    // 参数不需要使用逗号隔开。
    // 参数可以任意组合！
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

create_func!(fn_rules_p);

#[derive(Debug)]
struct G;

// macro_rules! impl_trait {

// }

macro_rules! exprs_to_hashmap {
    //这里可以之定义一些符号
    // ($(($key:expr,$value:expr)),+)可以使用 ("key","value")
    // ($($key:expr => $value:expr),+) => {可以使用 ("key","value")
    ($(($key:expr,$value:expr)),+) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert(stringify!($key), stringify!($value));
            )+
            map
        }
    };
}
#[macro_export]
macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

fn main() {
    fn_rules_println!();
    let hash_map: HashMap<_, _> = exprs_to_hashmap![
        ("key1", "value1"),
        ("key2", "value2"),
        ("key3", "value3") // "key1" => "value1",
                           // "key2" => "value2",
                           // "key3" => "value3"
    ];
    println!("{:?}", hash_map);
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    fn_rules_p();
}
