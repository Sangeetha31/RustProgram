use std::io;

fn main(){

    let mut input=String::new();
    println!("Enter First Number:");
    io::stdin().read_line(&mut input).unwrap();
    let a:i32=input.trim().parse().unwrap();
    
    input.clear();
    println!("Enter Second Number:");
    io::stdin().read_line(&mut input).unwrap();
    let b:i32=input.trim().parse().unwrap();
    
    input.clear();
    println!("Enter Third Number:");
    io::stdin().read_line(&mut input).unwrap();
    let c:i32=input.trim().parse().unwrap();
    
    if a>=b && a>=c{
        println!("{} is Bigger Than {} and {}",a,b,c);
    }
    else if b>c{
        println!("{} is bigger than {}",b,c);
    }
    else {
        println!("{} is biggest number",c);
    }
}