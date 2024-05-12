fn main() {
    let mut x=5;
    println!("the vlaue of x is : {}",x);
    x=6;
    println!("the value of x is : {}", x);

    let tup = (1,2.3,4);

    let (x,y,z)=tup;
    println!("value of y is {y}");
    println!("value of x is {x}");
    println!("value of z is {z}");
}
