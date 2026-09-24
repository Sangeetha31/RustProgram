use std::fs::*;
use std::path::Path;
use std::io::*;

fn main(){

    let fpath="demo.txt";
    if Path::new(fpath).exists(){
        let mut file=OpenOptions::new()
                     .append(true)
                     .open(fpath)
                     .unwrap();
        writeln!(file,"\nThis is New Line-1").unwrap();
        writeln!(file,"This is New Line-2").unwrap();
        writeln!(file,"This is New Line-3").unwrap();


    }
    else{
        println!("Incorrect File Path/File Not Found!");
    }
}