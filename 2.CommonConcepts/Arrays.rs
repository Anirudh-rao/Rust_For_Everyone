//Arrays in Rust

fn main (){
    let a =  [1,2,3,4,5];

    let first = a[0];
    let second= a [1];

    //Accessing Arrays
    let index = a[4] ;
    println!("The Value of element is :{}", index);

    //Invalid  - array out of bound
    //let index = 10;
    //let element = a[index];
    //println!("The value of element is: {}", element);
}