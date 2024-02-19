// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    fluid_oz: f64,
    flavor: Flavor,
}

fn print_drink (drink: Drink) {
    match drink.flavor {
        Flavor::Fruity => println!("flavor: Fruity"),
        Flavor::Sweet => println!("flavor: Sweet"),
        Flavor::Sparkling => println!("flavor: Sparkling")
    }

    println!("oz: {}", drink.fluid_oz);
}

fn main() {

    let drink: Drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 5.0
    };

    print_drink(drink);

    let drink: Drink = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 10.033
    };

    print_drink(drink);

    let drink: Drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 7.20
    };

    print_drink(drink);

}
