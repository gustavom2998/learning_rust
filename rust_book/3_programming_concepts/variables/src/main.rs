fn main(){
    // String: A string containing a sequence of spaces
    let spaces = "     "; 

    // Shadowing to redefine what spaces is
    // Integer: Number of space characters within the spaces string previously.
    let spaces = spaces.len();

    println!("{}",spaces);

    // String: A string containing a sequence of spaces
    let mut spaces = "     "; 

    // Integer: Number of space characters within the spaces string previously.
    // error: mismatched types - spaces is a string and spaces.len() is a int
    spaces = spaces.len();
}