fn main(){
    let width = 50;
    let height =  50;

    println!(
        "The Area of the Rectangle is {} Square Pixels"
        area(height,width);
    )
}
fn area(width:u32, height:u32) -> u32{
    width*height
}