#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn run(){
    println!("Created=================");
    // t::s::f::tsf();
    //CTFE
    
}
// mod  t{
//     pub(crate) mod  s{
//         pub(crate) mod f{
//          pub  fn tsf(){
//                 println!("这是模块")
//             }
//         }
//     }
// }