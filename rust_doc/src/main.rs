#! [allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;




fn main(){
    enum Day{
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl  Day {
        fn is_weekend(&self) ->bool{
            match self{
                Day::Saturday | Day::Sunday => true,
                _=> false
            }
        }
    }
    let today:Day=Day::Monday;
    match today{
        Day::Monday=>println!("Everyone Hates Monday"),
        Day::Tuesday =>println!("Donut day"),
        Day::Wednesday => println!("Hump Day"),
        Day::Thursday =>println!("Pay DAy"),
        Day::Friday=>println!("Almost Weekend"),
        Day::Saturday =>println!("Weekend"),
        Day::Sunday =>println!("Weekend"),
    }

    println!("Is today a weekend {}",today.is_weekend());

}