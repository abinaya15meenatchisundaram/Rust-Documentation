#! [allow(unused)]
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;

//hashmaps are used to store key value pairs

fn main() {

    struct Rectangle<T, U>{
        length:T,
        height:U,
    }
    let rec =Rectangle{length:4,height:10.5};
    trait Shape{
        fn new(length: f32,width:f32)-> Self;
            fn area(&self)-> f32;
        
    }

}
