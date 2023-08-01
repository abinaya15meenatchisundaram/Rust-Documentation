use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    let secret_number:i32=rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}",secret_number);
    loop{
        println!("Input the number");
        let mut guess=String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess:i32=guess.trim().parse().expect("Please type a number");
    
        println!("You guessed: {}",guess);
    
        match guess.cmp(secret_number){
            Ordering::Less=>println!("Too Less!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("you win");
                break;
        },
    }

}
}