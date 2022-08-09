use std::{thread, sync::mpsc::{self, Sender, Receiver}};

fn main() {
    //标准库线程的简单实用
    simple_std_thread();
    simple_std_thread_builder();
    //多线程消息发送通道
    std_misc();

}
fn simple_std_thread(){
    //使用标准库线程
    let join1 = thread::spawn(|| {
        for i in 0..2 {
            println!("线程一:{}", i)
        }
    });
    //使用标准库线程
    let join2 = thread::spawn(|| {
        for i in 0..2 {
            println!("线程二：{}", i)
        }
    });
    //这里又一些记过需要等待处理完成之后返回
    let _ = join1.join();
    let _ = join2.join();
}

//*配置标准库变成 */
fn simple_std_thread_builder(){
    let _=thread::Builder::new().name("simple_std_thread_builder".to_string()).spawn(move||{
        for i in 0..2 {
            println!("{:?}:{}",thread::current().name().unwrap() ,i)
        }
    }).unwrap().join();
}

static NTHREADS: i32 = 3;
fn std_misc(){
    //通道具有两个端点，分别是Sender<T>,Receiver<T>,T是需要发送数据的类型,(类型标注是可选的)
    let (tx,rx):(Sender<i32>,Receiver<i32>) = mpsc::channel();
    for id in 0..NTHREADS{
        //sender 端是可被复制的
        let thread_tx = tx.clone();

        //每一个线程都将通过通道来发送他的id

        thread::spawn(move||{
            //创建的线程去的thread_tx的所有权
            thread_tx.send(id).unwrap();
          // 发送是一个非阻塞（non-blocking）操作，线程将在发送完消息后
            // 会立即继续进行
            println!("thread {} finished", id);
        });
    }

      // 所有消息都在此处被收集
      let mut ids = Vec::with_capacity(NTHREADS as usize);
      for _ in 0..NTHREADS {
          // `recv` 方法从通道中拿到一个消息
          // 若无可用消息的话，`recv` 将阻止当前线程
          ids.push(rx.recv());
      }
       // 显示消息被发送的次序
    println!("{:?}", ids);
  
}