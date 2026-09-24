use std::fs;

fn main(){
    let filetext=fs::read("demo.txt").unwrap();
    println!("{}",filetext)
}