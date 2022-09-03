use std::{net::TcpStream, io::{Read,Write, stdout}};

fn main() {
   let mut  tcp_stream = TcpStream::connect("127.0.0.1:8024").expect("binde to ");
   let mut buff:[u8;2048] = [0;2048];
   let mut  strs = String::new();
   let mut isReg = false;
   loop {
    if isReg{
        println!("请输入用户名：");
        print!(">");
        let result  = std::io::stdin().read_line(&mut strs).unwrap();
        println!("结果:{},{}",result,strs.trim_end());
        tcp_stream.read(&mut buff).unwrap();
        //接受相应注册结果
        
    }
       
       
   }

}
