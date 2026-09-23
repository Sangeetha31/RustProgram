use std::io;

fn main(){

    loop{
        let mut inp=String::new();
        println!("Enter First Value");
        io::stdin().read_line(&mut inp).unwrap();
        let a:i32=inp.trim().parse().unwrap();

        inp.clear();

        let mut inp=String::new();
        println!("Enter Second Value");
        io::stdin().read_line(&mut inp).unwrap();
        let b:i32=inp.trim().parse().unwrap();

        inp.clear();

        println!("1.Add\n2.Sub\n3.Mul\n0.Exit\nEnter Your Choice:");
        io::stdin().read_line(&mut inp).unwrap();
        let choice:i32=inp.trim().parse().unwrap();

        match choice{
            1=>println!("{}",a+b),
            2=>println!("{}",a-b),
            3=>println!("{}",a*b),
            0=>{println!("Exiting Program!");break;},
            _ =>println!("Invalid choice"),
        }

    }
}