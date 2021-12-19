use core::time;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use  std::thread;
use std::thread::Thread;
use std::time::Duration;
use std::net::SocketAddr;
use  std::net::IpAddr;
use std::net::Ipv4Addr;
use std::io::Read;
use databases::databases;

// lazy_static!{

// }
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
//    let fb =  closure::fba(50);
//    let handler = thread::spawn(fun_thread);
//    handler.join().unwrap();
//    //启动一个socket服务端
//   if let Err(e) = newTcpServer(){
//     panic!("TCP服务器启动失败!")
//   }
databases::sqlite::connex();

}

fn fun_thread(){
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
fn handle_client(mut stream:TcpStream)->std::io::Result<()>{
   // println!("ip:{:?}",stream.peer_addr());
    let mut buf = [0; 512];
    // let mut buf_string = String::new();
    let  bytes_read = stream.read(&mut buf);
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(()); 
        }
        stream.write(&buf[..bytes_read])?;
    }
}

fn newTcpServer()->std::io::Result<()>{
    let listener = std::net::TcpListener::bind("127.0.0.1:3000")?;
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                    });
                }
            }
    }
    Ok(())
}