fn char_dec(){
    // Declare a char 
    let c = 'z';

    // Declare another char
    let emj = '😻';
}

fn try_mult_char_dec(){
    // Wont compile - chars can only be single unicode values
    //let c = 'zc';
    
    // Using double quotes declares as string, which is allowed
    let c = "zc";
}

fn main(){
    char_dec();
    try_mult_char_dec();
}
