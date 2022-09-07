use std::fmt::Display;

use  serde;
use serde_json::{self, json};


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
    josn_serialize_default();
}


//字段自定义序列化名称
//#[serde(rename = "name")]
// #[serde(rename(serialize = "ser_name"))]
// #[serde(rename(deserialize = "de_name"))]
// #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
#[derive(serde::Serialize,serde::Deserialize,Debug)]
struct  Dog{
   #[serde(rename="color")]
    color:String,
    breed:String,
    jump:bool,
    walk:bool,
    //此处用于解决解析json过程中缺省的值，默认default
    #[serde(skip_serializing_if = "ignore_lc_equal_0", default = "default_lc")]
    lc: f64,
}

impl  Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"breed:{},color:{},jump:{},walk:{}",self.breed,self.color,self.jump,self.walk)
    }
}




/**
 * 对于默认没有的json字段进行序列化
 */
fn josn_serialize_default(){
    let dog =r#"{
        "color":"black",
        "breed":"中华田园犬",
        "walk":true,
        "jump":true
    }"#;
   let dog :Dog = serde_json::from_str(dog).unwrap();
   //如果需要使用{:?}` (or {:#?} 输出结构，则需要使用drive(Debug)
   println!("Dog{:?}",dog)

}

fn ignore_lc_equal_0(lc: &f64) -> bool {
    *lc == 0.0
}
fn default_lc() -> f64 {
    0.0
}
