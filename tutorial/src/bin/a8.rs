enum Flavor {
    Vanilla,
    Strawberry,
    Chocolate,
}

struct Drink {
    flavor: Flavor,
    quantity: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Vanilla => println!("Flavor: vanilla"),
        Flavor::Chocolate => println!("Flavor: chocolate"),
        Flavor::Strawberry => println!("Flavor: Strawberry"),
    }
    println!("Quantity: {:?} litres", drink.quantity);
}

fn main() {
    let my_smoothie = Drink {
        flavor: Flavor::Vanilla,
        quantity: 20.032,
    };
    print_drink(my_smoothie);
}
