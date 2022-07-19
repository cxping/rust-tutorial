#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub fn match_fn(){
    let x :&Option<i32> = &Some(3);
    if let Some(f) = x  {
        println!("模式匹配结果值{}",f)
    }
    //匹配对应类型
    let dog = Animal::Dog;
    match  dog {
        Animal::Monkey =>println!("孙悟空"),
        Animal::Dog =>  println!("哮天犬"),
        Animal::Pig => println!("小猪佩奇"),
        Animal::Chicken => println!("卯天星君"),
    }
}
//模式匹配 结果上的匹配 解构

pub enum Animal {
    Monkey,
    Dog,
    Pig,
    Chicken
}