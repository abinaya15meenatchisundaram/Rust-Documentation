#! [allow(unused)]
use std::io;
use std::cmp::Ordering;

fn print_str(x:String){
    println!("A String :{}",x);
}

fn print_return_str(x:String) ->String{
    println!("String : {}",x);
    x
}

fn change_str(name: &mut String){
    name.push_str(" is Happy");
    println!("Message : {}",name);
}

fn main() {
    let  mut str1=String::from("Friends");
    let  mut str2=str1.clone();
    println!("Welcome {}",str1);
    print_str(str1);
    let mut str3=print_return_str(str2);
    println!("{}",str3);
    change_str(&mut str3);
}
