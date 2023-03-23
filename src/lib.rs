use std::fs;
use serde_json::{Value, from_reader};


pub struct Json {
    pub data: Value,
}


impl Json {

    pub fn new(jsondata: &str) -> Self {
        Self {
            data: serde_json::from_str(jsondata).unwrap(),
        }
    }

    pub fn open(path: &str) -> Self {
        let file = fs::File::open(path).unwrap();
        Self {
            data: from_reader(file).unwrap(),
        }
    }

    pub fn save(&self, path: &str) {
        let file = fs::File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &self.data).unwrap();
    }

    pub fn disp(&self) {
        println!("{:?}", self.data);
    }
    
    pub fn set_value(&mut self, key: &str, value: Value) {
        let obj = self.data.as_object_mut().unwrap();
        obj.insert(key.to_string(), value);
    }
}



pub trait Set<T> {
    fn set(&mut self, key: &str, value: T);
}

impl Set<i32> for Json {
    fn set (&mut self, key: &str, value: i32) {
        self.set_value(key, Value::Number(serde_json::Number::from(value)));
    }
}

impl Set<String> for Json {
    fn set (&mut self, key: &str, value: String) {
        self.set_value(key, Value::String(value));
    }
}

impl Set<Json> for Json {
    fn set (&mut self, key: &str, value: Json) {
        self.set_value(key, value.data);
    }
}


pub fn to_string(val: Value) -> String {
    val.as_str().unwrap_or("").to_string()
}

pub fn to_i32(val: Value) -> i32 {
    val.as_i64().unwrap_or(0) as i32
}


/*
fn main() {
    let mut jso = Json::open("test.json");

    jso.set("country", "USA".to_string());
    jso.set("members", 102);
    jso.disp();


    let jsondata = r#"
    {
        "text": "Hello, world!"
    }
    "#;


    let mut jso = Json::new(jsondata);


    jso.set("name", "taro".to_string());
    jso.set("age", 16);
    jso.disp();

    jso.save("save.json");

    let name = jso.get::<String>("name");
    println!("{}", name);
    println!("{}", to_i32(jso.data["array"][2].clone()));
    let buf = to_string(jso.data["name"].clone());
    println!("{}", buf);

}
*/
