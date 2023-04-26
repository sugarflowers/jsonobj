use std::fs;
use serde_json::Value;
use sjis::{is_sjis, decode};

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
        let br = binaryfile::BinaryReader::open(path).unwrap().read().unwrap();
        let json_data: String; 
        if is_sjis(&br) {
            json_data = decode(br);
        } else {
            json_data = String::from_utf8(br).unwrap();
        }
        Self {
            data: serde_json::from_str(&json_data).unwrap(),
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

    pub fn keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        if let Some(obj) = self.data.as_object() {
            for key in obj.keys() {
                keys.push(key.to_string());
            }
        }
        keys
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

#[test]
fn test_json() {
    let mut jso = Json::open("test_sjis.json");

    let name = to_string(jso.data["name"].clone());
    let age = to_i32(jso.data["age"].clone());

    println!("{} ({})", name, age);

    jso.set("age", 25);
    let age = to_i32(jso.data["age"].clone());
    println!("{} ({})", name, age);
}

#[test]
fn new_json() {
    let mut jso = Json::new("{}");
    jso.set("name", "hanako".to_string());
    let name = to_string(jso.data["name"].clone());
    println!("{}", name);
    jso.save("test2.json");
}

#[test]
fn keys_test() {
    let jso = Json::open("test.json");
    println!("{:?}", jso.keys());
}

