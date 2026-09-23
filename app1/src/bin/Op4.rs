fn main(){
    let mut x=100;
    println!("Before Fn Call:{}",x);

    show_number(&mut x);

    println!("After Fn Call:{}",x);
}

fn show_number(a:&mut i32)
{
    *a=*a+1;
}