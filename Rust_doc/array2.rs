let arry=[1,2,3,4,5,6,7,8,9];
let mut idx=0;
loop{
    if arry[idx]%2==0{
        idx+=1;
        continue;
    }
    if arry[idx]==9{
        break;
    }
   
    println!("Val {}",arry[idx]);
    idx+=1;
}