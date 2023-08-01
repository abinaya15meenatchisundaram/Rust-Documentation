// Define a struct called "Person"

fn main() {
    //Standard if-else
    let num = 10;

    if num > 0 {
        println!("The number is positive.");
    } else {
        println!("The number is non-positive.");
    }


    //Multiple if-else branches:
    let age=18  ;
    if age> 18{
        println!("You are eligible to vote");
    } else if age<18 {
        println!("You are not eligible to vote");
    } else {
        println!("You got the right to vote!");
    }


    //if-else as expression:
    let num = 10;
    let result = if num > 0 {
        "positive"
    } else {
        "non-positive"
    };
    
    println!("The number is {}", result);

}