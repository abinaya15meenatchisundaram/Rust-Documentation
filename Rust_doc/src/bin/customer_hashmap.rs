#! [allow(unused)]
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;

//hashmaps are used to store key value pairs

fn main() {
     struct customer{
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob=customer{
        name:String::from("Bob Smith"),
        address:String::from("555 Street"),
        balance:234.50,
    };
    bob.address=String::from("505 Main St");
    

    

}
