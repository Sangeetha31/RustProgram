use std::collections::HashMap;
fn main(){
    let mut courses=HashMap<(i32,S&str)>::new();

    courses.insert(10201,"Java");
    courses.insert(10101,"SQL");
    courses.insert(20580,"Python");
    courses.insert(00250,"Oracle");


    courses.insert(10101,"Mongo");
    courses.insert(90201,"Java");
    println!("{:?}",courses);

    println!("Course Name:{:?}",courses.get(&20580));

    courses.remove(&00250);
    println!("{:?}",courses);


}