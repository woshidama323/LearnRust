use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
pub fn test(){
    println!("what is it ");
}


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    address: String,
}
pub struct Example {
    pub number: i32,
}
impl Example {
    pub fn boo(){
        println!("hello boo");
    }

    pub fn answer(&mut self) {
        self.number += 32;
        println!("current number{:?} ",self.number);
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }

    //测试anyhow
    pub fn get_cluster_info() -> Result<Person> {

        let config = std::fs::read_to_string("config.json")?; // use ? to easily propagate any error that implements the std::error::Error trait.
        let map: Person = serde_json::from_str(&config)?;
        Ok(map)
    }
}