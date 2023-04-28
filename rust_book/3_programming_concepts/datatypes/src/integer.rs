fn explicit_declaration(){

    // Success - unsigned int containing the value 42
    let val: u32 = "42".parse().expect("Not a number!");
    println!("{} declared as string and parse as int", val);

    // error: 
    //let val = "42".parse().expect("Not a number!");
}

fn literal_declaration(){
    // 255 declaration in decimal
    let mut val:u8 = 255;
    println!("{} declared in decimal", val);

    // 255 declaration in hex
    val = 0xff;
    println!("{} declared in hex", val);

    // 255 declaration in octal
    val = 0o377;
    println!("{} declared in octal", val);

    // 255 declaration in binary
    val = 0b11111111;
    println!("{} declared in binary", val);
}

fn wrapping_operation(){

    // Success - unsigned int containing the value 255 (upper limit)
    let mut val: u8 = 255;

    // Sucess - wrapping add around to zero - 0 than 255
    val = val.wrapping_add(1);

    // Prints zero
    println!("{} - 255 as 8 bit int + 1 with wrapping add", val);
    val = 255;

    // error: this arithmetic operation will overflow
    val = val + 1;
}

fn main(){
    explicit_declaration();
    literal_declaration();
    wrapping_operation();
}