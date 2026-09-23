struct Employee{
    id:i32,
    name: String,
    salary:f64
}

impl Employee{
    fn display(&self){
        println!("ID: {}",self.id);
        println!("Name: {}",self.name);
        println!("Salary: {}", self.salary);
    }
}
fn main(){
    let emp1=Employee{
        id:1,name:String::from("Ram"),salary:65000.0
    };
    emp1.display();

}