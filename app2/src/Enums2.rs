struct Employee{
    id:i32, name:String, department:String, status:EmployeeStatus
}

enum EmployeeStatus{
    Active,
    Onleave,
    Resigned
}

fn print_employee(emp: &Employee){
    println!("Id: {}",emp.id);
    println!("Name: {}",emp.name);
    println!("Department: {}",emp.department);

    match &emp.status{
        EmployeeStatus::Active=>{
            println!("Status: Active");
        },
        EmployeeStatus::Onleave=>{
            println!("Status: On leave");
        },
        EmployeeStatus::Resigned=>{
            println!("Status: Resigned");
        }
    }
}
fn main(){
    let emp=Employee{
        id:1235, name:String::from("Harry"),department:String::from("HR"),
        status: EmployeeStatus::Active
        };
        print_employee(&emp);
}