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
}
//模式匹配 结果上的匹配 解构

