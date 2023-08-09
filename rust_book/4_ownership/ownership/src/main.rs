
fn mutable_string(){
    let mut s = String::from("hello"); // Define mutable string

    s.push_str(" world!"); // Change string content in run time - add " World" to the end of the string

    println!("{}", s); // Print "Hello World!"
}

fn main() {
    mutable_string();
}
