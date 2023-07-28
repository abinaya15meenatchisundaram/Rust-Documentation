#! [allow (unused)]
use std::io;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let vec1: Vec<i32>=  Vec::new();
    let mut vec2=vec![1,2,3,4];
    vec2.push(5);
    println!("1st {}",vec2[0]);
    let second:&i32=&vec2[1];
    match vec2.get(1){
        Some(second)=>println!("2nd:{}",second),
        None=>println!("No 2nd value"),
    }
    for i in &mut vec2{
        *i *=2;

    }
    for i in  &vec2{
        println!("{}",i);
    }
    println!("Vec Length {}", vec2.len());
    println!("POP : {:?}",vec2.pop());

    }

    fn main() {
        let mut my_vec=vec![1,2,3,4];
        println!("{}",my_vec[1]);
        my_vec.push(5);
        my_vec.remove(1);
        
        for i in my_vec.iter(){
            println!("The vector is {}",i);
        }
    }
    
