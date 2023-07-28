fn main() {
    let mut my_string=String::from("Hello Friends this is Abi");
    println!("Length: {}",my_string.len());
    println!("Is empty {}",my_string.is_empty());
    
    for i in my_string.split_whitespace(){
        println!("{}",i);
    }
    println!("Does my string contains 'Abi' {}",my_string.contains( "Abi"));
    my_string.push_str(" Welcome to my repo");
    println!("{}",my_string);
    }
    