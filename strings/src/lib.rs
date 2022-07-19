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

    #[test]
    fn modify_string_test(){
        crate::modify_string();
    }
}


/**
 * 
 * 1:从字面量中创建字符串
 * 
 */
fn new_string(){
    //创建一个空字符串
    let str1 = String::new();
    //字符切片中创建一个String类型
    let str2  = String::from("你好呀");
    let s = "Hwllo world".to_string();
    let s1:String = "小猪佩奇乔治".into();
    println!("字面量创建:{}", str2);
    println!("字面量创建:{}", s);
    println!("字面量创建:{}", s1);
    let bytes :&[u8] = b"\xE4\xB8\x96\xE7\x95\x8C";
    //byte数组转换为字符串
    let str3 =String::from_utf8(bytes.to_vec());
    println!("{:?}",str3);
    let str4 = String::from("Hello world");
    //将字符转为char数组
    let chars:Vec<char> = str4.chars().rev().collect();
    println!("{:?}",chars);

    //拼接 字符  message

    let s = "Hello".to_string();
    let message = s +"hello";
    println!("{}",message);

    //接受一个有效UTF-8
      let sparkle_heart = vec![240, 159, 146, 150];
      let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
      println!("{}",sparkle_heart);
      let bytes = sparkle_heart.into_bytes();
      println!("bytes:{:?}",bytes);
    /*
      let s :str= "Hello world";
    let s :Box<str> = "Helloworld".into();
    */

}


fn modify_string(){
    let  mut  s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += &"!".to_string();

    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats");

    // 字符串或者字符数组:字节数组
    let bytes_string:&[u8;22] = b"this is  a bytes array";
    println!("A byte string: {:?}", bytes_string);

    // 字符串索引string index
    // &s1[start..end]
    //start 和 end 必须准确落在字符的边界处
    let s1 = String::from("world");
    let s2 = &s1[2..4];
    let s3 = &s1[0..1];
    println!("{},s3:{}",s2,s3)
   


}