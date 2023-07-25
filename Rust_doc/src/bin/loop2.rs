fn main() {
    let mut counter = 0;
//loop: Creates an infinite loop until explicitly exited using the break statement

    loop {
        println!("Loop iteration: {}", counter);
        counter += 1;
    
        if counter >= 5 {
            break;
        }
    }
}