use std::panic;

fn main(){
    let result=panic::catch_unwind(||{
        println!("some code execution");
        panic!("OOPS Something Went Wrong!");
    });

    match result{
        Ok(_)=>println!("execution completed successfully!"),
        Err(_)=>println!("Panic was Handled"),
    }

    println!("Job Completed!");
}