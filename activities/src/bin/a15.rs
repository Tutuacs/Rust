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
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

impl Ticket {
    fn new(ticket: Ticket) -> Self {
        {
            ticket
        }
    }

    fn print(&self) {
        match self {
            Ticket::Backstage(any_price, any_name) => println!("\nTicket type: Backstage \nTicket price: {}\nTicket name: {}", any_price, any_name),
            Ticket::Vip(any_price, any_name) => println!("\nTicket type: Vip \nTicket price: {}\nTicket name: {}", any_price, any_name),
            Ticket::Standard(any_price) => println!("\nTicket type: Backstage \nTicket price: {}", any_price)
        }
    }
}

fn main() {

    let mut tickets: Vec<Ticket> = vec![Ticket::new(Ticket::Backstage(50.99, "Arthur Silva".to_owned()))];

    let joas_ticket: Ticket = Ticket::Standard(5.98);

    let brenda_ticket: Ticket = Ticket::new(Ticket::Vip(100.70, "Brenda".to_owned()));

    tickets.push(brenda_ticket);
    tickets.push(joas_ticket);

    for ticket in &tickets {
        ticket.print();
    }

    println!("\nTickets on total: {}\n",tickets.len());

}
