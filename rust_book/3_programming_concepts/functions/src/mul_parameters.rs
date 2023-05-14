
fn main(){
    // 1st print
    println!("Hello world");
    // Call another function where cost=1.5f and currency=R$
    another_function(1.5, '$');
}

// Definition for another function
fn another_function(cost: f64, currency: char){
    println!("The values passed to another function: {}{}", currency, cost);
}