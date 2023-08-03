use std::collections::HashMap;
fn main() {
    let mut marks=HashMap::new();

    //Add values
    marks.insert("Rust Programming",96);
    marks.insert("Python Programming",98);
    marks.insert("Internet of Things",95);
    marks.insert("Mathematics",80);

    println!("How many subjects you studied?{}",marks.len());

    match marks.get("Python Programming"){
        Some(mark)=>println!("You got {} marks",mark),
        None => println!("You did'nt study Python Programming")
    }

    //remove a value
    marks.remove("Mathematics");

    //loop through HashMap
    for (subject,mark) in &marks{
        println!("For {} You got {}%",subject,mark);
    }

    //Check for value
    println!("Did you studied Java? {}",marks.contains_key("Java"));


}