struct color{
    red:u8, //0-255 
    green:u8,
    blue:u8,
}
struct nums(u8,u8,u8);

fn main(){
    let mut one=nums(1,2,3);
    one.2=60;
    println!("One is {}, {}, {},",one.0,one.1,one.2);

    let mut bg= color{red:255,green:70,blue:15};
    bg.red=45; 
    println!("{}, {}, {}",bg.red,bg.green,bg.blue);
}