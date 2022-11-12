//Tuples in Rust
//Tuples Belong to the Compound Data types

fn main(){
    //One Way is to define alll the data types
    let tup :(i32, f64,u8) = (500,6.4,1);

    //One more method
    let tup = (500,6.4,1);

    //We Can bind the entire Tup to one element
    let (x,y,z) = tup;

    println!("The Values of y is{}",y);


    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


}