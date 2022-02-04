struct Person {
    name: String,
    age: i32,
    favourite_color: String,
}

fn print_name(name: &str) {
    println!("name: {name}");
}

fn print_favourite_color(color: &str) {
    println!("favourite_color: {color}");
}

fn main() {
    let people = vec![
        Person {
            name: "Ernest".to_owned(),
            age: 20,
            favourite_color: "Blue".to_owned(),
        },
        Person {
            name: "Obama".to_owned(),
            age: 13,
            favourite_color: "Yellow".to_owned(),
        },
        Person {
            name: "Arianna".to_owned(),
            age: 8,
            favourite_color: "Pink".to_owned(),
        },
        Person {
            name: "Phill".to_owned(),
            age: 7,
            favourite_color: "Purple".to_owned(),
        },
    ];

    for person in &people {
        if person.age < 10 {
            print_name(&person.name);
            print_favourite_color(&person.favourite_color);
        }
    }
}
