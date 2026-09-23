fn main(){
    let mut stack:Vec<i32>=Vec::new();

    stack.push(25);
    stack.push(35);
    stack.push(45);
    stack.push(55);

    println!("{:?}",stack);

    stack.pop();
    stack.pop();
    println!("{:?}",stack);
}