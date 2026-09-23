use std::io;
fn main(){
    let mut val1=String::new();
    let mut val2=String::new();

    println!("Enter First value");
    io::stdin().read_line(&mut val1).unwrap();


    println!("Enter Second value");
    io::stdin().read_line(&mut val2).unwrap();

    let x:f32=val1.trim().parse().unwrap();
    let y:f32=val2.trim().parse().unwrap();

    let res=sum(x,y);
    println!("The Sum is {} and {} is {:.2}", x,y,res);
}

fn sum(a:f32,b:f32)->f32{
   a+b
}