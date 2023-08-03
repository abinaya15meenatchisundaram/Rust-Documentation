extern crate rand;
use rand::Rng;


fn main(){
    let random_num=rand::thread_rng().gen_range(1..11);
    println!("Random Number:{}",random_num);

    //Flip a coin
    let random_bool=rand::thread_rng().gen_weighted_bool(2);
    println!("Random_Boolen:{}",random_bool);
}