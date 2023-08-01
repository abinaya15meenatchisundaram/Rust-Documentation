#! [allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
let num_1: u32= 5;
let num_2:u32=4;
println!("5+4 = {}",num_1+num_2);
println!("5-4 = {}",num_1 -num_2);
println!("5*4 = {}",num_1*num_2);
println!("5/4 = {}",num_1/num_2);
println!("5%4 = {}",num_1%num_2);
}