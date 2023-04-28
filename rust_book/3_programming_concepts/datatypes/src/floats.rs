fn float_declaration(){
    // 64 bit float
    let x = 2.0; 

    // 32 bit float
    let y: f32 = 3.0;
}

fn special_floats(){
    let mut x = 1.0;

    // Infinity
    x = x / 0.0;
    println!("{}", x);

    // Negative infinity
    x = x * (-1.0);
    println!("{}", x);

    // Negative zero
    x = -0.0;
    println!("{}", x);

    // NaN
    let mut x:f64 = -1.0;
    x = x.sqrt();
    println!("{}", x);
}

fn main(){
    float_declaration();
    special_floats();
}