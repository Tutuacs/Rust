// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {

    let my_string = "Minha String".to_owned();
    
    let my_upper_string = my_string.to_uppercase();
    let my_lower_string = my_string.to_lowercase();

    println!("\nNormal: {}",my_string);
    println!("\nUpper: {}",my_upper_string);
    println!("\nLower: {}",my_lower_string);
}
