use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}


fn new_vec() {
   //创建一个Vec集合
    let vec = Vec::<u8>::new();
    let vec2 = vec![1, 2, 3, 4, 5, 6];
}
fn feach_vec() {
    let  mut vec = vec![1, 2, 3, 4, 5, 6];
    //1:常规方式
    // for ele in vec {
    //     println!("{}",ele);
    // }
    //下列均为迭代器
    //2:通过迭代器的方式
    // for ele in vec.iter() {
    //     println!("{}",ele);
    // }
    //2:遍历带下标的方式
    // for (index,ele) in vec.iter().enumerate() {
    //     println!("Index:{},ele:{}",index,ele);
    // }


    //3:for_each
    // vec.iter().for_each(|f|{
    //     println!("{}",f);
    // });


    // 4:next 
    // while let Some(e) = vec.iter().next() {
    //     println!("{}",e);
    // }


    //iter_mut :提供了一个可变引用的迭代器
    // for ele in vec.iter_mut() {
    //     *ele+=1;
    // }


    //可以对数组进行翻转，然后进行迭代
    vec.iter().rev().for_each(|f|{
        println!("{}",f)
    });
}

/// 对集合的修改操作
fn insert_vec(){
    let mut vec  = vec![2,3,4];
    //集合尾部添加数组
    vec.push(10);
    //通过下标在集合中插入数据
    vec.insert(1, 11);


    //合并2个数组
    //方式一
    // let vec2 = vec![10,11,23,43,45];
    // vec.extend(vec2);
    // println!("{:?}",vec);


    //方式二 将另一个数组，追加进来
    // let  mut vec2 = vec![10,11,23,43,45];
    // vec.append(&mut vec2);
    // vec.align_to()

}


fn vec_qeque(){ //双向队列的简单使用
    //从集合中创建双向队列
    // let mut vec  = vec![2,3,4];
    // let qeque:VecDeque<_> = vec.into_iter().collect();
    let mut quque:VecDeque<i32> = VecDeque::new();
    // quque.insert(index, value)
    quque.push_back(1);
    quque.push_back(2);
    quque.push_back(3);
    quque.push_back(4);
    quque.push_back(5);

    quque.push_front(10);
    println!("{:?}",quque);

}
