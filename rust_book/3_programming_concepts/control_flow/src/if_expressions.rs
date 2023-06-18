fn basic_if(){
    let number = 3;
    if number < 5 {
        println!("Number is smaller than 5");
    }
    else {
        println!("Number is not smaller than 5");
    }
}

fn multiple_ifs(){
    let number = 13;

    if number % 2 == 0 {
        println!("Number is divisible by 2");
    }
    else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }
    else if number % 4 == 0 {
        println!("Number is divisible by 4");
    }
    else {
        println!("Number is not divisible by 2,3 or 4.");
    }
}

fn ternary(){
    let condition = true;
    let number = if condition {1} else {0};
    println!("Number: {number}")
}


fn main(){
    basic_if();
    multiple_ifs();
    ternary();
}
