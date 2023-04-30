// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct GroceryItem {
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &GroceryItem) {
    println!("ID: {}", item.id);
}
fn main() {
    let item = GroceryItem {
        quantity: 1,
        id: 1,
    };
    display_quantity(&item);
    display_id(&item);
}
