struct student{
    name:String,
    age:u8,
}

impl ToString for student{
    fn to_string(&self)-> String{
        return format!("My name is {} and my age is {}",self.name,self.age);
    }
}

fn main() {
    let stu= student{name:String::from("Jimin"),age:28};
    println!("{}",stu.to_string());
}
