fn main(){
    let str1=String::from("Helllooo!!, ");
    let str2=String::from("My dear son!");
    let str3= str1+ &str2;
    println!("{}",str3);
}