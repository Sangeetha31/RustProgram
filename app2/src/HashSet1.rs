use std::collections::*;

fn main(){
    let mut scores=HashSet::new();

    scores.insert(65);
    scores.insert(32);
    scores.insert(90);
    scores.insert(44);
    scores.insert(65);

    scores.remove(&90);
    println!("{:?}", scores);

    println!("Size:{}",scores.len());
}