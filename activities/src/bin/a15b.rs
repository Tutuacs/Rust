#[derive(Debug)]
enum Port {
    Big(String),
    Medium(String),
    Small(String),
}

#[derive(Debug)]
struct Dogs {
    name: String,
    port: Port,
    age: u8,
}

impl Dogs {
    fn new(name: String, port: Port, age: u8) -> Self {
        Self { name, port, age }
    }
}

fn main() {
    let big_dog = Dogs {
        name: "Bradok".to_owned(),
        port: Port::Big("Alemao".to_owned()),
        age: 7,
    };

    let mut dogs_list: Vec<Dogs> = vec![
        Dogs::new(
            "Iza".to_owned(),
            Port::Medium("Boiadeiro Australiano".to_owned()),
            3,
        ),
        Dogs::new(big_dog.name, big_dog.port, big_dog.age),
    ];

    let age: u8 = 1;
    let port: Port = Port::Small("Daushaunds".to_owned());
    let name: String = "Zoe".to_owned();

    dogs_list.push(Dogs::new(name, port, age));

    for dog in dogs_list {
        println!("\nCachorros: {:?}", dog);
    }
}
