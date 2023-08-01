fn main() {
    let mut n = 0;
    //while: Repeats the code block as long as a given condition is true.
    
    while n < 5 {
        if n % 2 != 0 && n% 3 !=0 && n % 5 !=0 && n% 7 !=0{
            println!("{} is a prime number",n)
        } else {
            println!("{} is not a prime number",n)
        }
        // Increment counter
        n += 1;
    }
}