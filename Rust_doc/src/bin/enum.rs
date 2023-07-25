/*
an enum (short for "enumeration") is a data type 
that allows you to define a type that can have a fixed set of distinct values, known as variants. 
Enums can only take one of a limited number of possible values.variable can take one of several predefined values, 
and each value can have associated data or be unit-like without any data.
*/

/* The match expression is similar to a switch or case statement. 
It allows you to match a value against a series of patterns and 
execute code corresponding to the matched pattern. */
 enum fruits{
    Apple,
    Mango,
 }

enum Direction{
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction:Direction=Direction::Down;

    match player_direction{
        Direction::Up =>println!("We are heading upwards!"),
        Direction::Down => println!("We are going down...."),
        Direction::Left => println!("We are moving towards left.."),
        Direction::Right =>println!("We are going in right direction"), 
        __=>println!("Nothing selected")
    }
}
