#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    // #[test]
    // fn it_task() {
    //    crate::run();
    // }

    
}
pub mod future_study;

// use std::{time::Duration, thread};

// use async_std::{prelude::*, task};

// async fn say_hello(name:&str){
//     println!("hello world,=>{}",name);
    
// }

// fn run(){
//     task::block_on(async{
//         loop {
//             say_hello("第一个").await;
//             thread::sleep(Duration::from_millis(100));
//             say_hello("===============================>").await;
//             thread::sleep(Duration::from_millis(100));
//         }
//     });
// }