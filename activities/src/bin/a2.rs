// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {

    let a = 5;
    let b = 5;

    let result = sum(a, b);

    show_sum(result);

}

fn show_sum(result: i32){
    println!("The result is: {}",result);
}

fn sum(first_num: i32, second_num: i32) -> i32{
    first_num + second_num
}