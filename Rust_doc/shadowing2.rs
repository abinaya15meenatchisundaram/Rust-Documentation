fn main(){
    let mut x=10;
    {
        let x=15;
        println!("x is {}",x);
    }
    println!("X : {}",x);

    let x="x is a string now";
    println!("x : {}",x);

    let x=true;
    println!("x : {}",x);
}