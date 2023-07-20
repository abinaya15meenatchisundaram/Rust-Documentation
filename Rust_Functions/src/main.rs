#! [allow(unused)]
use std::io;
use std::fmt::Display;

//function just prints
fn say_hello(){
    println!("Hello Friends Lets work on Functions");
}

//function that just prints sum not returns value
fn get_sum(x:i32,y:i32){
    println!("The sum of the two numbers are without returning value");
    println!("{} + {} = {}",x,y,x+y)
}

//function that returns value
fn get_sub(x:i32,y:i32) -> i32{
    println!("The diffrence of the two numbers with return values");
    x - y
}

fn get_sub2(x:i32,y:i32) -> i32{
    println!("Difference using return");
    return x - y;
}

fn get_2(x:i32) ->(i32,i32){
    return (x+1,x+2);
}
fn main() {
    let (val_1,val_2)=get_2(13);
    say_hello();
    get_sum(5,4);
    println!("Difference {}",get_sub(5,4));
    println!("Difference {}",get_sub2(5,4));
    println!("Get 2 Nums : {} {}",val_1,val_2);


}
