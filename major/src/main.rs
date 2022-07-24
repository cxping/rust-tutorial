use core::time;
use  std::thread;
use asyncs;

fn main() {
    // let a = 10;
    // let b = 11;
    // let s = "你好";
    // let s1 = String::from("霓虹");
    // println!("Hello, world!");
    // created::run();
    // //  过程宏+build脚本(build.rs)
    // closure::close();
    // matching::match_fn();
    // created::run();
    //let fb =  closure::fba(50);
    //    let handler = thread::spawn(fun_thread);
    //    handler.join().unwrap();
    //    //启动一个socket服务端
    //   if let Err(e) = newTcpServer(){
    //     panic!("TCP服务器启动失败!")
    //   }
    // databases::sqlite::connex();
    // meshwork::newTcpServer();
    // if let Err(err) =meshwork::clinet::run(){
    //     panic!("客户端运行失败：{}",err);
    // }
    // loop {
    //    thread::sleep( Duration::from_secs(1));
    // }

    asyncs::future_study::futures_launch();
    loop {
        println!("main_launch======>");
        thread::sleep(time::Duration::from_millis(1000))
    }
}