fn main(){
    let a = [10,20,30,40,50];
    let mut index =0;

    while index  <5{
        println!("The Value is  {}", a[index]);
        index =  index+1;
    }

    //Using the Itermetod
    for element in a.iter(){
        println!("The Values is  {}", element);
    }
    
    //using the rev method
    //Will Count from 4-1
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}