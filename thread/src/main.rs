use std::{thread, sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex}};

fn main() {
    //标准库线程的简单实用
    simple_std_thread();
    simple_std_thread_builder();
    //多线程消息发送通道
    std_misc();
    //重复执行任务
    task_run();

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





// 一个任务执行demo
fn task_run() {
    let task_info = Arc::new(Mutex::new(TaskInfo{
        timer:10,
        task:fun1,
        run_state:0
    }));

    let task_clone  = task_info.clone();
    thread::spawn(move||{
        loop {
            let  mut task  = task_clone.lock().unwrap();
            if task.run_state ==0{ //任务处于等待执行中
                task.timer = task.timer-1;
                if task.timer >0{
                    task.run_state =1;
                }
            }
            thread::sleep(std::time::Duration::from_millis(1))
        }
    });

    let task_clone2  = task_info.clone();
    loop {
        let   mut task  = task_clone2.lock().unwrap();
        if task.run_state ==1{
            (task.task)();
            task.run_state =0;
            task.timer =10;
        }
        thread::sleep(std::time::Duration::from_micros(1))
    }
}

#[derive(Debug,Clone)]
struct  TaskInfo
{
    timer:i32,
    task:fn(),
    run_state:i32
}


pub fn fun1(){
    println!("任务一");
}
pub fn fun2(){
    println!("任务二");
}
pub fn fun3(){
    println!("任务三");
}