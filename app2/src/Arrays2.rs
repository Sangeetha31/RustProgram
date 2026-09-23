use std::io;

fn main(){
    
     let mut arr=[0;100];

    let mut inp=String::new();

    println!("Enter Array Size");
    io::stdin().read_line(&mut inp).unwrap();
    let size:usize=inp.trim().parse().unwrap();

    //read values into array
    for i in 0..size{
        println!("Enter Value for Index-{}:",i);

        let mut arrinp=String::new();
        io::stdin().read_line(&mut arrinp).unwrap();
        arr[i]=arrinp.trim().parse().unwrap();

    }

    //printing array elements
    println!("{:?}",arr);

    //printing array elements
    for i in 0..size{
        print!("{} \t",arr[i]);
    }

    println!("");
}