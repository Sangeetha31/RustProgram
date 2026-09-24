use std::fs::File;


fn main(){
    let res=File::create("demo.txt");

    match res{
        Ok(_)=>println!("File Created Succesfully!"),
        Err(error)=>println!("Error:{}",error)
    }
}