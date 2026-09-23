trait PrintData{
    fn print(&self);
}

struct Student{
    id:i32, name:String,age:i32
}

struct Course{
    id:i32, name:String,duration:i32
}

impl PrintData for Student{
    fn print(&self){
        println!("Student: Id: {}, Name: {}, Age: {}",self.id,self.name,self.age)
    }
}

impl PrintData for Course{
    fn print(&self){
        println!("Student: Id: {}, Name: {}, Duration: {}",self.id,self.name,self.duration)
    }
}

fn ShowData(data: &impl PrintData)
{
    data.print();
}


fn main(){
    let st=Student{
        id:101, name:String::from("James"),age:26
    };

    let cr=Course{
        id:201, name:String::from("Java"),duration:1
    };

    ShowData(&st);
    ShowData(&cr);
    st.print();
    cr.print();

    let mut data:&dyn PrintData;
    data=&st;
    data.print();
    
    data=&cr;
    data.print();


}