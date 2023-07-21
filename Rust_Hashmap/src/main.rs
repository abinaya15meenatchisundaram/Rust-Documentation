#! [allow(unused)]
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;

//hashmaps are used to store key value pairs

fn main() {
    let mut heroes= HashMap::new();
    heroes.insert("Spiderman","Tom Holland");
    heroes.insert("Ironman","Tony Stark");
    heroes.insert("Captain","Steven");

    for (k,v) in heroes.iter(){
        println!("{} = {}",k,v);
    }
    println!("Length : {}",heroes.len());
    if heroes.contains_key(&"Ironman"){
        let the_Ironman=heroes.get(&"Ironman");
        match the_Ironman{
            Some(x)=>println!("Spiderman is a Hero"),
            None=> println!("Spiderman is not a hero"),
        }
    }
}
