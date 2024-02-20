// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct People {
    name: String,
    favorite_color: String,
    age: u8,
}

impl People {
    fn new(name: String, favorite_color: String, age: u8) -> Self {
        Self {
            name,
            age,
            favorite_color,
        }
    }

    fn print(&self) {
        if self.age <= 10 {
            println!("Name: {}, age: {}, favorite color: {}", self.name, self.age, self.favorite_color);
        }else{
            println!("Age upper than 10 years");
        }
    }

}


fn main() {
    let name = "Arthur".to_owned();
    let favorite_color = "Black".to_owned();
    let age = 9;
    
    let mut vet_people = vec![People::new(name, favorite_color, age)];

    let name = "Joas".to_owned();
    let favorite_color = "Blue".to_owned();
    let age = 10;

    let new_people = People::new(name, favorite_color, age);
    vet_people.push(new_people);

    let name = "Brenda".to_owned();
    let favorite_color = "Green".to_owned();
    let age = 11;

    let new_people = People::new(name, favorite_color, age);

    vet_people.push(new_people);

    for person in vet_people {
        person.print();
    }

}
