fn main() {
    // Remove the keyword let and this script will fail
    let mut x = 5;
    println!("The value of x is: {x}");
    
    // Error message: cannot assign twice to immutable variable
    x = 6;
    println!("The value of x is: {x}");
}