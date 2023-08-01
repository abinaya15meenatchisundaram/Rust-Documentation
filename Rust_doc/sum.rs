//function that just prints sum not returns value   //return and print
fn get_sum(x:i32,y:i32) ->i32{
    let mut c:i32;
    c=x+y;
    c
}

fn main() {

    println!("{}",get_sum(5,4));
}