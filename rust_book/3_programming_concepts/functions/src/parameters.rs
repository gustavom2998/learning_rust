
fn main(){
    // 1st print
    println!("Hello world");
    // Call another_function to print x=1
    another_function(1);
}

// Definition for another function
fn another_function(x: i32){
    // 2nd print - print x
    println!("The value passed to another function: {x}");
}