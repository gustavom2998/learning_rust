fn main() {

    // Will work - normal const definition
    const FIVE: u32 = 5;

    // Reassign value - wont work
    // error: cannot assign to this expression
    FIVE = FIVE + 1;

    // Will fail consts can't be declared with mut keyword
    // error: const globals cannot be mutable
    const mut TEN: u32 = 10;
}