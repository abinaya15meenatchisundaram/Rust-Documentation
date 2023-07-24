#! [allow(unused)]
use std::io;
use std::fmt::Display;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let my_age=18;
    let voting_age=18;
    match my_age.cmp(&voting_age){
        Ordering::Less=>println!("Can't Vote"),
        Ordering::Greater=>println!("Can vote"),
        Ordering::Equal=>println!("You can vote now"),
    };

}
