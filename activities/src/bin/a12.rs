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
    Red,
    Green,
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

struct Box {
    color: Color,
    dimensions: Dimensions,
    weight: f64,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Green => println!("The color is Green"),
            Color::Red => println!("The color is Red"),
            // _ => println!("Unknown color")
        }
    }
}

impl Dimensions {
    fn print(&self) {
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("depth: {}", self.depth);
    }
}

impl Box {
    fn new(weight:f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {}", self.weight);
        println!("- - - - - - - - - - - ");
    }
}


fn main() {

    let dimension_small = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };

    let dimension_medium = Dimensions {
        width: 2.0,
        height: 3.0,
        depth: 4.5,
    };

    let dimension_big = Dimensions {
        width: 5.0,
        height: 3.0,
        depth: 5.0,
    };

    let small_box = Box::new(5.0, Color::Green, dimension_small);
    let medium_box = Box::new(10.0, Color::Red, dimension_medium);
    let _big_box = Box::new(20.0, Color::Green, dimension_big);

    small_box.print();
    medium_box.print();
    _big_box.print();

}
