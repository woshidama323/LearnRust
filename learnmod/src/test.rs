pub fn test(){
    println!("what is it ");
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
}

