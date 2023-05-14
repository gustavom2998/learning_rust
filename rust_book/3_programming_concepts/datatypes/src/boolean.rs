fn boolean_ops(){
    let x:bool = true;
    let y:bool = false;

    println!("{}", x && y); // false
    println!("{}", x || y); // true
    println!("{}", x ^ y); // true
    println!("{}", !x); // false
}