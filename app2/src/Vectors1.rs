fn main(){
    let mut courses:Vec<&str>=Vec::new();

    courses.push("Java");
    courses.push("Angular");
    courses.push("Vue");
    courses.push("React");
    courses.push("NodeJS");

    courses.remove(3);
    courses.pop();
    println!("{:?}",courses);

    let mut uicourses:Vec<String>=Vec::new();

    uicourses.push(String::from("Node JS"));
    uicourses.push(String::from("Angular JS"));
    uicourses.push(String::from("React"));
    uicourses.push(String::from("Vue"));

    println!("{:?}",uicourses);

    let mut x:&str="Hello";
    x="ABC";
    println!("{}",x);

    let mut y:String=String::from("india");
    y.push_str("abc");
    println!("{}",y);


}