fn main() {
    let s  = String::from("hardik");

    println!("{}",s);  // working

    foo(s);

    println!("{}",s); // should not work
}

fn foo(s: String){
    println!("{}",s);
}
