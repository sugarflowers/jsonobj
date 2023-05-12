use std::path::PathBuf;
//use std::env;
use std::fs;

pub struct PathObj {
    pub path: PathBuf,
}

impl PathObj {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }

    pub fn push(&mut self, value: &str){
        self.path.push(value);
    }

    pub fn join(&mut self, values: Vec<&str>) {
        for value in values {
            self.path.push(value);
        }
    }

    pub fn from_str(&mut self, val: &str) {
        let sval: Vec<&str> = val.split(std::path::MAIN_SEPARATOR).collect();
        self.join(sval);
    }

    pub fn pop(&mut self) {
        self.path.pop();
    }

    pub fn parent(&mut self) -> String {
        let path = self.path.parent().unwrap();
        format!("{}", path.display()) 
    }

    pub fn file_name(&mut self) -> String {
        let path = self.path.file_name().unwrap();
        format!("{}", path.to_string_lossy())
    }

    pub fn extension(&mut self) -> String {
        let path = self.path.extension().unwrap();
        format!("{}", path.to_string_lossy())
    }
    
    
    pub fn get(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn set(&mut self, path: &str) {
        self.path = PathBuf::from(path);
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub fn getcwd(&mut self) {
        let current = std::env::current_dir().unwrap();
        self.path = current;
    }

    pub fn get_home(&mut self) {
        let home = env::get_env("USERPROFILE");
        self.set(&home);
    }

    pub fn get_temp(&mut self) {
        let temp = env::get_env("TEMP");
        self.set(&temp);
    }

    pub fn is_exists(&self) -> bool {
        if let Ok(_m) = fs::metadata(&self.path) {
            true
        } else {
            false
        }
    }
}


#[test]
fn new_function_test() {
    let mut p = PathObj::new();
    p.getcwd();
    println!("{:?}", p.is_exists());
}
#[test]
fn str_path() {
    let mut p = PathObj::new();
    p.from_str("a\\b\\c");
    println!("{:?}", p.parent());
}
#[test]
fn home_dir() {
    let mut p = PathObj::new();
    p.get_home();
    println!("{:?}", p.get());
    println!("{:?}", p.parent());
}
