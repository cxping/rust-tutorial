use std::{thread, time::Duration, net::TcpStream, io::{Read, Write}};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub mod clinet{
    use std::io::{Result, Write, Read};
    use std::net;
    pub fn run()->Result<()>{
        //打开与远端主机的链接
        let mut client = net::TcpStream::connect("127.0.0.1:3000")?;
        //接收控制数据输入
       let mut redBuf:[u8;512] =[0;512];
       //1:发送链接数据
       let  comment = String::from(r#"{"id":"1000","client":"1","namespace":"connect"}"#);
       //发送客户端链接注册参数
        client.write(comment.as_bytes())?;
        client.flush()?;
       //2:接收用户控制数据
       let mut  input =std::io::stdin();
       //3:读取对端响应数据
    //    loop { 
    //     //循环等待用户输入数据
    //     let mut input_str = String::from("");
    //     input.read_to_string(&mut input_str)?;

    //    }
        Ok(())
    }
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
            println!("{}",bytes_read);
            return Ok(()); 
        }
        stream.write(&buf[..bytes_read])?;
    }
}

pub fn newTcpServer()->std::io::Result<()>{
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