use  serde;
use serde_json;


#[derive(Debug,serde::Serialize,serde::Deserialize)]
struct  Persion{
    age:u8,
    name:String,
    address:String,
    sex:u8,
    salary:f64,
}

impl  Persion {
    fn new(age:u8,name:&str,sex:u8,address:&str,salary:f64)->Self{
        Persion { age,name: name.to_string(), address: address.to_string(), sex, salary }
    }
    //将对象转换为字符串
    fn to_json(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
    //解析字符穿为对象
    fn from_value(json:&str)->Persion{
        serde_json::from_str(json).unwrap()
    }
}

pub fn json_simple(){
    let persion = Persion::new(20,"雾里飞花",10,"群仙六星计",1333.0);
    println!("{:?}",persion);
    println!("{:?}",persion.to_json());


}