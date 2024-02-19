// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum COLOR {
    RED,
    GREEN,
    BLUE,
    YELLOW
}

fn main() {

    let my_favorite_color: COLOR = COLOR::GREEN;

    match my_favorite_color {
        COLOR::BLUE => println!("Your favorite color is blue"),
        COLOR::GREEN => println!("Your favorite color is green"),
        COLOR::RED => println!("Your favorite color is red"),
        COLOR::YELLOW => println!("Your favorite color is yellow"),
    }


}
