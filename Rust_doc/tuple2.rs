fn main() {
    let tuple = (5u32,);
    println!("One element tuple: {:?}", tuple);

    // Create a new tuple with additional elements
    let new_tuple = (tuple.0, 10u32, "Hello");
    println!("Three element tuple: {:?}", new_tuple);
}