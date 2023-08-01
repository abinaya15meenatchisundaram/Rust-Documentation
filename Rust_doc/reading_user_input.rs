use std::io;

fn main() {
    let mut word=String::new();
    println!("Hey mate! say something:");
    match io::stdin().read_line(&mut word){
        Ok(_)=>{
            println!("Success! You said:{}",word);
        },
        Err(e)=> println!("Oops Something went wrong: {}",e)
    }
}
