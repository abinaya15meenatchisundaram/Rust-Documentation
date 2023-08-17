fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..11 {
        if n % 8 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}