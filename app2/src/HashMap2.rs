use std::io;
use std::collections::*;
fn main(){
    let mut employees:HashMap<(i32,String)>=HashMap::new();
    let mut inp=String::new();

    println!("Enter No. of Employees:");
    io::stdin().read_line(&mut inp).unwrap();
    let size:usize=inp.trim().parse().unwrap();

    for i in 0..size{
        inp.clear();
        println!("Enter Id:");
        io::stdin().read_line(&mut inp).unwrap();
        let id:i32=inp.trim().parse().unwrap();

        inp.clear();
        println!("Enter Name");
        io::stdin().read_line(&mut inp).unwrap();
        let name:i32=inp.trim().to_string();

        employees.insert(id, name);
    }

    for (k,v) in employees{
        println!("{}.{}",k,v);
    }
}