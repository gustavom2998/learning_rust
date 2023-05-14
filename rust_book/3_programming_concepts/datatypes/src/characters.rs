fn char_dec(){
    let c = 'z';
    let emj = '😻';
}

// Wont compile - chars can only be single unicode values
// Using double quotes declares as string, which is allowed
fn try_mult_char_dec(){
    //let c = 'zc';
    let c = "zc";
}

fn main(){
    char_dec();
    try_mult_char_dec();
}
