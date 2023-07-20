#! [allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;



fn main(){ 
    let s=String::from("a b c d e f g h i j k k l m");
    let mut v1:Vec<char>=s.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1{
        println!("{}",char);
    }
    let s2="Random Sring";
    let mut s3:String=s2.to_string();
    println!("{}",s3);
    let byte_arr1=s3.as_bytes();
    let s4=&s3[0..6];
    println!("{}",s4);
    println!("String length:{}",s4.len());
    s3.clear();
    let s5=String::from("Just Some");
    let s6=String::from("Words");
    let s7=s5+ &s6;
    for char in s7.bytes(){
        println!("{}",char);
    }
}
