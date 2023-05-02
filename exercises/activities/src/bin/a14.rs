// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: u8,
    name: String,
    favorite_color: String,
}fn print_person(person: &Person) {
    println!("{}: {}", person.name, person.favorite_color);
}
fn main() {
    let people = vec![
        Person {
            age: 10,
            name: String::from("Bob"),
            favorite_color: String::from("blue"),
        },
        Person {
            age: 20,
            name: String::from("Sally"),
            favorite_color: String::from("green"),
        },
        Person {
            age: 30,
            name: String::from("Jane"),
            favorite_color: String::from("red"),
        },
    ];

    for person in &people {
        if person.age <= 10 {
            print_person(person);
        }
    }
}
