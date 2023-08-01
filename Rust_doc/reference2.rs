fn main(){
    let mut x=20;
    println!("X before changing: {}",x);
{
    let y= &mut x;
    *y +=1;
    println!("y after changing {},",y);
    println!("X inside the scope {}",x);
}
    println!("x outside the scope {}",x);
}