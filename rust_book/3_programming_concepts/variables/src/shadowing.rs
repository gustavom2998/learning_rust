fn main() {

    // Define a non-mutable variabel
    let x = 5;

    // Shadow the variable by using let again
    let x = x + 1;

    {
        // Shadow the variable a second time by using let again
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}