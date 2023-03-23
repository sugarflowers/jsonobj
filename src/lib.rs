/*
 * This file is part of PROJECT.
 *
 * PROJECT is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * PROJECT is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with PROJECT.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::PathBuf;
use std::env;

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
    
    pub fn getcwd() -> String {
        let current = env::current_dir().unwrap();
        format!("{}", current.display()) 
    }
    
}

/*

use pathobj::PathObject;

let mut path = PathObject::new();
path.push(&t);
path.push("aaa");

path.join(vec!["abc", "def", "ghi.png"]);

println!("{}", path.parent());
println!("{}", path.file_name());
println!("{}", path.extension());
println!("{}", PathObject::getcwd());

 */
