// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let my_numbers = vec![1,2,3,4];

    for number in &my_numbers {
        print_vec(&number);
    }

    print_total(my_numbers);
}

fn print_vec(number: &i32) {
    match number {
        30 => println!("Thirty"),
        _ => println!("{}", number),
    }
}

fn print_total(vec: Vec<i32>) {
    println!("Total: {}", vec.len())
}
