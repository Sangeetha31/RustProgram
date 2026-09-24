struct Address{
    city:String, state:String
}

struct Employee{
    id:i32, name:String,address: Address
}

impl Employee{
    fn display(&self){
        println!("Id: {}", self.id);
        println!("Name: {}", self.name);
        println!("City: {}", self.address.city);
        println!("State: {}", self.address.state);

    }
}

fn main(){
    let emp1=Employee
    {
        id:101,
        name:String::from("Harish"),
        address: Address{
            city: String::from("Pune"),
            state: String::from("MH")
        }
    };
    emp1.display();
}