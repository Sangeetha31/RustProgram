fn main(){
    let x:i32=-10;
    println!("{}",x.abs());
    println!("{}",x.is_positive());
    println!("{}",x.is_negative());

    let i:i32=2;
    println!("{}",i.pow(3));

    let f:f32=25.0;
    println!("{}",f.sqrt());

    let f1:f32=10.5;
    println!("{}",f1.floor());
    println!("{}",f1.ceil());
    println!("{}",f1.round());

    let i1:i32=25;
    println!("{}",i1.min(20));
    println!("{}",i1.max(20));
}