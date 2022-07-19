//数组类型 [T;长度]
//数组的长度是类型签名的一部分，数组长度是不可变的，因此数组长度必须要在编译期知道
//数组，只是长度不可变，声明为可变类型时。数组内容时允许修改的
pub fn create_array() {
    let i32_arr:[i32;512] = [0;512];
    // let  u8_arr:[u8;64] =[0;64];
    // u8_arr[0] = 1; //
    let mut  u8_arr:[u8;8] =[0;8];
    u8_arr[0] =100;
    u8_arr[7] =100;
    // thread 'main' panicked at 'index out of bounds:
    // the len is 8 but the index is 63', collections/src/arrays.rs:9:5
    // u8_arr[8] =100; 越界索引会导致代码的 panic
    //
    println!("u8_arr:{:?}",u8_arr);

    //数组切片
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];

    println!("slice:{:?}",slice);
    println!("size_of_val:{:?}",std::mem::size_of_val(&slice));
    assert!(std::mem::size_of_val(&slice) == 16);
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice:&[i32] = &arr[1..4];
    println!("{:?}",slice);
    assert_eq!(slice, &[2, 3, 4]);


    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);
}
