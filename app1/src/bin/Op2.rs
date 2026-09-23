use std::io;
fn main(){

    let mut first=String::new();
    let mut last=String::new();

    println!("Enter Your First Name:");
    io::stdin().read_line(&mut first).unwrap();
    
    println!("Enter Your Last Name:");
    io::stdin().read_line(&mut last).unwrap();

    Greet(first.trim().to_string(), last.trim().to_string());
}

fn Greet(fname:String, lname:String){
    println!("Welcome, {} {}",fname,lname);
}