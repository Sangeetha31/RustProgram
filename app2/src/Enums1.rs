enum CourseStatus{
    NotStarted,
    InProgress,
    Cancelled,
    Completed
}

fn main(){
    let status=CourseStatus::Completed;

    match status{
        CourseStatus::NotStarted=>{
            println!("Course has not started");
        }
        CourseStatus::InProgress=>{
            println!("Course is in Progress");
        }
        CourseStatus::Cancelled=>{
            println!("Course is Cancelled");
        }
        CourseStatus::Completed=>{
            println!("Course is Completed");
        }
    }
}