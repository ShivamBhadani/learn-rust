// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, f32),
    Vip(String, f32),
    Standard(f32),
}

impl Ticket {
    fn print(&self) {
        match self {
            Ticket::Backstage(name, price) => {
                println!("Backstage ticket for {} costs ${}", name, price);
            }
            Ticket::Vip(name, price) => {
                println!("VIP ticket for {} costs ${}", name, price);
            }
            Ticket::Standard(price) => {
                println!("Standard ticket costs ${}", price);
            }
        }
    }
}

fn main() {
    let tickets = vec![
        Ticket::Backstage("John".to_string(), 100.0),
        Ticket::Vip("Jane".to_string(), 200.0),
        Ticket::Standard(50.0),
    ];

    for ticket in tickets {
        ticket.print();
    }
    
}
