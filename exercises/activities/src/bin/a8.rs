// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
enum Flavor {
    Orange,
    Grape,
    Apple,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_ounces: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_drink_info(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("Orange"),
        Flavor::Grape => println!("Grape"),
        Flavor::Apple => println!("Apple"),
    }
    println!("{} ounces", drink.fluid_ounces);
}
// * Use a match expression to print the drink flavor

fn main() {
    let orange_drink = Drink {
        flavor: Flavor::Orange,
        fluid_ounces: 10.0,
    };
    let grape_drink = Drink {
        flavor: Flavor::Grape,
        fluid_ounces: 12.0,
    };
    let apple_drink = Drink {
        flavor: Flavor::Apple,
        fluid_ounces: 8.0,
    };

    print_drink_info(orange_drink);
    print_drink_info(grape_drink);
    print_drink_info(apple_drink);
}
