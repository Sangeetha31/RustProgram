struct InsufficientFunds;

fn withdraw(balance: f64, amount:f64)->Result<f64,InsufficientFunds>{
    if amount>balance{
        Err(InsufficientFunds)
    }
    else{
        Ok(balance-amount)
    }
}

fn main(){
    let res=withdraw(15000.00, 7000.00);

    match res{
        Ok(balance)=>println!("Transaction is Succesfull. Balance Available: {}",balance),
        Err(_)=>println!("Error: Insufficient Funds in Account")
    }
}