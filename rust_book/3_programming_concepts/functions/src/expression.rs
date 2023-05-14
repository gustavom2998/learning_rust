fn main(){
    // The brackets evaluate to 4
    let y = {
        let x = 3;

        // No semi colon - if there was, would be a statement
        x + 1
    };

    // Should print 4
    println!("Y: {y}");
}