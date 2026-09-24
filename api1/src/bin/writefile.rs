fn main(){

    let fpath="demo.txt";
    if Path::new(fpath).exists(){
        fs::write(fpath,"This is Sample Text").unwrap();
    }
    else{
        println!("Incorrect File Path/File Not Found!")
    }
}