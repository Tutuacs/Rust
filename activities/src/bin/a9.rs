// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let (_x, y): (i32, i32) = return_tuple();

    if y > 5 {
        println!("> 5");
    }else if y < 5 {
        println!("< 5");
    }else {
        println!("== 5");
    }

}

fn return_tuple() -> (i32, i32) {
    (2, 3)
}

// fn print_coord(item: (i32, i32)) {
//     loop {
//         println!("First: {}, Second {}",item.0, item.1);
//     }
// }
