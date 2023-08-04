fn main(){
    let my_string = String:: from("Rust is fantastic");
    println!("After replace: {}",my_string.replace("fantastic","great"));

    {
        let my_string=String::from("The weather is \nnice\noutside mate!");
        for line in my_string.lines(){
            println!("[ {} ]",line);
        } 
    }
}