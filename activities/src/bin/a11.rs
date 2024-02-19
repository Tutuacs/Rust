// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    id: i8,
    quantity: i8,
}

fn set_item() -> Item {
    Item {
        id: 1,
        quantity: 10
    }
}

fn display_id(item: &Item) {
    println!("The id is {}", item.id);
}

fn display_quantity(item: Item) {
    println!("The quantity is {}", item.quantity);
}


fn main() {
    let first_item: Item = set_item();

    display_id(&first_item);
    display_quantity(first_item);

}

