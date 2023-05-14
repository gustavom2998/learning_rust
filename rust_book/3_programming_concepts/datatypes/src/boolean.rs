fn boolean_ops(){
    let x:bool = true;
    let y:bool = false;

    // Logical AND 
    println!("{}", x && y); // false

    // Logical OR
    println!("{}", x || y); // true

    // Logical XOR
    println!("{}", x ^ y); // true

    // Logical NOT
    println!("{}", !x); // false
}