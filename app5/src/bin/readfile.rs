use std::fs;

fn main(){
    let filetext=fs::read_to_string("demo.txt").unwrap();
    println!("{}",filetext)
}