#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
/*
//!1:如果没有任何捕获变量，则实现FnOnce
//!2:如果有捕获变量，并且会对捕获的变量进行修改，则是实现FnMut
//!3:如果有补货殡殓刚，并且不会对捕获的变量进行修改，则实现Fn
//!特殊情况
//!1:编译器回讲FnOnce当成fn(T)函数指针对待
//!2；Fn/FnMut/FnOnce 这三者trait 的关系是依次继承，他们正好对应所有权语义三件套
close
闭包实现Copy/Clone的原则
1:如果环境变量实现了copy，闭包如果可以借用方式捕获环境变量，并对其进行修改，则闭包自身不会实现Copy
2:如果环境变量是Move语义，则闭包内补货环境变量的操作，及修改环境变量或者是消耗环境变量，则必自身不会实现Copy
*/

pub fn close(){
    //关于闭包的使用
    let ff = |i:i32|{i+10};
    println!("闭包:{}",ff(10 ));
    let mut arr = [1,2,3,4];
    let mut c2 = |i|{
        arr[0]=i;
        println!("{:?}",arr);
    };
    // c2(10);

}
//FnOnce 
//FnMut
//返回一个fn闭包函数执政类型
fn cls_fn()->impl Fn()->i32{
    ||{10}
}
//返回可变类型的闭包
//逃逸闭包
//非逃逸闭包
fn c_mut()->impl for<'a> FnMut(&'a str)->String{
    //这里如果使用&str类型，因为咩有实现copy的trait，所以闭包内部是无法取得s的所有权，
    //这里是存方在堆里面的数据
    // let   s = "hello".to_string();
    //离开函数后就失效了，
    move |i|{
         let mut  a = "hello world".to_string();
         a +=i;
         a
    }
}

pub  fn inc(n:i32)->i32{
    n+1
}