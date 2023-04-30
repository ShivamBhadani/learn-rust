// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
// * Use a function to display the result
fn display_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, sum(a, b));
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    display_sum(1, 2);
    display_sum(3, 4);
    display_sum(5, 6);
}
