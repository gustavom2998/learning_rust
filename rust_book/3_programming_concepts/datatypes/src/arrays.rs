fn declaration(){
    // Declare an array with explicit types and by manually specifying its values
    let x: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", x)
}

fn constant_declaration(){
    // Declare an array with explicit types and constant values
    let x = [0; 5];
    println!("{:?}", x)
}

fn indexing(){
    let x: [i32; 5] = [1,2,3,4,5];

    // Access the first element of an array
    println!("{}", x[0]);
}

fn bad_indexing(){
    let x: [i32; 5] = [1,2,3,4,5];
    // Access an index thats larger than the array (will generate an error)
    println!("{}", x[6]);
}