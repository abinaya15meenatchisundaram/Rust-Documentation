#! [allow (unused)]
use std::io;
use std::cmp::Ordering;
use::std::ops::Add;

fn get_sum_gen<T:Add<Output =T>>(x:T,y:T) ->T{
    return x + y; //we cannot add by x+y in generics
}

fn main() {
    println!("Sum 5 + 4 = {}",get_sum_gen(5,4));
    println!("Sum 5.2 + 4.6 = {}",get_sum_gen(5.2,4.6));
}
