use chrono::{Datelike, Duration, Local, Months, Timelike};

fn main(){
    
    let dt=Local::now();
    println!("Current Date and Time:{}",dt);

    println!("Current Date and Time in Custom Format:{}",dt.format("%d-%m-%Y %H:%M:%S"));
    println!("Current Date and Time in Custom Format:{}",dt.format("%d-%m-%Y %H:%M:%S %p"));

    println!("Year:{}",dt.year());
    println!("Month:{}",dt.month());
    println!("Day:{}",dt.day());
    println!("WeekDay:{}",dt.weekday());

    println!("Hour: {}",dt.hour());
    println!("Minutes: {}",dt.minute());
    println!("Seconds: {}",dt.second());

    let future=dt+Duration::days(40);
    println!("Future Date:{}", future.format("%d-%m-%Y"));

    let future1=dt+Months::new(10345);
    println!("Future date:{}",future1.format("%d-%m-%Y"));

    let future2=dt+Duration::weeks(10345);
    println!("Future date:{}",future2.format("%d-%m-%Y"));

    
    let future2=dt+Duration::hours(10345);
    println!("Future date:{}",future2.format("%d-%m-%Y %H:%M:%S"));
    
    let future2=dt+Duration::minutes(10345);
    println!("Future date:{}",future2.format("%d-%m-%Y %H:%M:%S"));

}