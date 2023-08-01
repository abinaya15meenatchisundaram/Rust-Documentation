fn get_2(x:i32) ->(i32,i32){
    return (x+1,x+2);
}

fn main() {
    let (val_1,val_2)=get_2(13);  
    println!("Get 2 Nums : {} {}",val_1,val_2);
}