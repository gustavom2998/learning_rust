fn five() -> i32 {
    // No ; for expression
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // Will return () - no value - error since it must return i32
    // x + 1; 
}

fn main(){
    println!("Five: {}", five());

    println!("Five plus one: {}", plus_one(5));

}   