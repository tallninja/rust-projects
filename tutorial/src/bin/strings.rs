struct Person {
    name: String,
    age: i32,
}

fn print_person_name(name: &str) {
    println!("name: {name}");
}

fn main() {
    let people = vec![
        Person {
            name: "Ernset".to_owned(),
            age: 22,
        },
        Person {
            name: String::from("Obama"),
            age: 13,
        },
        Person {
            name: "Arianna".to_owned(),
            age: 8,
        },
    ];

    for person in &people {
        print_person_name(&person.name);
        println!("age: {:?}", person.age);
    }

    println!("{:?}", people.len());
}
