#! [allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    const ONE_MIL: u32=1_000_000;
    const PI:f32=3.141592;
    let age: &str= "47";
    let mut age: u32=age.trim().parse()
        .expect("Age Wasn't Assigned a Number");
    age=age+1;
    println!("I'm {} and I want ${}",age,ONE_MIL);
}
