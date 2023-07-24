#! [allow(unused)]
use std::io;
use std::fmt::Display;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
let age = 22;
    match age{
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX =>println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };
}
