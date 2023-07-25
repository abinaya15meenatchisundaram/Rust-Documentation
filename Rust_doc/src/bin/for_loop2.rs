fn main() {
//For-In Loop: for in iterable collections
    let numbers = [1, 2, 3];
    println!("For-In Loop");
    for number in &numbers {
            println!("{}", number);
    }

//For-Range Loop: to iterate over a range of values.
    println!("For-Range Loop");
    for i in 1..3 {
            println!("{}", i);
    }

}