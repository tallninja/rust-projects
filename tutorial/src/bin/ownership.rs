// enum Display {
//     Dark,
//     Light,
// }

// fn display_light(light: Display) {
//     match light {
//         Display::Dark => println!("dark"),
//         Display::Light => println!("light"),
//     };
// }

// fn main() {
//     let light = Display::Light;
//     display_light(light);
//     display_light(light);
// }

enum Display {
    Dark,
    Light,
}

fn display_light(light: &Display) {
    match light {
        Display::Dark => println!("dark"),
        Display::Light => println!("light"),
    };
}

fn main() {
    let light = Display::Light;
    display_light(&light);
    display_light(&light);
}
