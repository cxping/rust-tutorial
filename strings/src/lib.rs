use std::string;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn new_string_test(){
        crate::new_string();
    }
}


fn new_string(){
    //创建一个空字符串
    let str1 = String::new();
    //字符切片中创建一个String类型
    let str2  = String::from("你好呀");
    let bytes :&[u8] = b"\xE4\xB8\x96\xE7\x95\x8C";
    //byte数组转换为字符串
    let str3 =String::from_utf8(bytes.to_vec());
    println!("{:?}",str3);

    let str4 = String::from("Hello world");
    //将字符转为char数组
    let chars:Vec<char> = str4.chars().rev().collect();
    println!("{:?}",chars);
    

}