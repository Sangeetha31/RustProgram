use std::collections::LinkedList;

fn main(){

    let mut llist=LinkedList::new();

    llist.push_back(25);
    llist.push_back(33);
    llist.push_back(11);
    llist.push_back(40);
    llist.push_back(22);

    println!("{:?}",llist);

    llist.pop_front();
    llist.pop_back();
    println!("{:?}",llist);
}