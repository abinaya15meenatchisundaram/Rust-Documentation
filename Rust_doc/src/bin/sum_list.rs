fn sum_list(list:&[i32]) -> i32{
    let mut sum=0;
    for &val in list.iter(){
        sum+=&val;
    }
    sum
}
fn main() {
    let num_list=[1,2,3,4,5];
    println!("Sum of list = {}",sum_list(&num_list));

}