use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::channel,
    thread, env,
};
use dotenv::dotenv;
use log::info;
fn main() {
    //加载环境配置
    dotenv().ok();
    //添加日志输出篇日志
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    //添加绑定服务端口
    let address = std::env::var("SERVER_URL").unwrap_or("127.0.0.1:8024".to_string());
    info!("binding to:{}",address);
    //绑定服务端口
   let tcp_listener = TcpListener::bind(address).expect("绑定服务端口失败");

   //遍历获取等待所有的链接
    tcp_listener.incoming().filter_map(Result::ok).for_each(|mut stream|{
        thread::spawn(move ||{
            let mut buff:[u8;1024] = [0;1024];
            loop {
                //
                let n = stream.read(&mut buff).unwrap_or_default();
                if n==0{
                    continue;
                }
                //解析数据
                info!("接受的数据:{:?}",buff);
            }
        });
   });
   
}
