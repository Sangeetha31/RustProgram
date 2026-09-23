fn main(){

    let dt=Local::now();

    let s4=dt.format("%d %m %Y").to_string();
    println!("{}",s4);

    let k:i32=16;
    let l:f64=k as f64;
    println!("{:.2}",1);

    let f1:f64=128.256;
    let i1:i32=f1 as i32;
    println!("{}",i1);
}