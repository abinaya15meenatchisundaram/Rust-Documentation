#! [allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let age: i32=60;
    if (age >=1) && (age<=18){
        println!("Very Important Birthday");
    } else if (age ==21) || (age==50){
        println!("Important Birthday");
    } else if (age>=65){
        println!("Yes is it also important Birthday");
    } else {
        println!("Not Important Birthday");
    }
}
