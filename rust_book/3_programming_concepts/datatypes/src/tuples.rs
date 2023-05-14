fn declaration(){
    // Declare a tuple with explicit types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Print tuple
    println!("{:?}", tup)
}

fn pattern_match(){
    // Declare a tuple with explicit types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructure tuple into separate variables for each element
    let (x,y,z) = tup;

    // Print an element
    println!("{}", x)
}

fn indexing(){
    // Declare a tuple with explicit types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Access the first element of a tuple and print it
    println!("{}", tup.0)
}

fn main(){
    declaration();
    pattern_match();
    indexing();
}

