// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Brown,
    Black,
    White,
}
impl Color{
    fn print(&self){
        match self {
            Color::Brown => println!("Brown"),
            Color::Black => println!("Black"),
            Color::White => println!("White"),
        }
    }
}
struct ShippingBox {
    dimensions: (i32, i32, i32),
    weight: i32,
    color: Color,
}
impl ShippingBox {
    fn new(dimensions: (i32, i32, i32), weight: i32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print(&self) {
        println!(
            "Dimensions: {:?}, Weight: {}, Color: ",
            self.dimensions, self.weight
        );
        self.color.print();
    }
}
fn main() {
    let box1 = ShippingBox::new((1, 2, 3), 4, Color::Brown);
    box1.print();
}
