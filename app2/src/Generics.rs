use std::fmt::Display;

struct Course<T>{
    id:T,
    name:String,
}

impl<T: Display> Course<T>{
    fn print(&self){
        println!("Id: {}",self.id);
        println!("Name: {}",self.name);
    }
}

fn main(){
    let course1=Course::<i32>{
        id:101,
        name:String::from("Angular")
    };

    let course2=Course::<&str>{
        id:"UI2589",
        name:String::from("Node JS")
    };
    println!("Course1: Id:{} Title:{}",course1.id, course1.name);
    println!("Course2: Id:{} Title:{}",course2.id, course2.name);

    course1.print();
    course2.print();
}