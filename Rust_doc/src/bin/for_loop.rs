#! [allow(unused)]
use std::io;
use std::fmt::Display;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let arry=[1,2,3,4,5,6,7,8,9];
    for i in arry.iter(){
        println!("Val {}",i)
    }
}
