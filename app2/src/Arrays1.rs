fn main(){
    let mut arr1=[10;3];
    arr1[0]=105;
    arr1[1]=136;
    arr1[2]=785;
    println!("{:?}",arr1);

    let mut courses=["Java","Rust","React","Node","Angular","Python"];
    courses[0]="Go";
    println!("{:?}",courses);

    courses.sort();
    println!("{:?}",courses);

    courses.reverse();
    println!("{:?}",courses);

}