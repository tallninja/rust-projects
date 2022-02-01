enum Color {
    Black,
    White,
    Blue,
    Brown,
}

fn main() {
    let color: Color = Color::Blue;
    match color {
        Color::Black => println!("black"),
        Color::White => println!("white"),
        Color::Blue => println!("blue"),
        Color::Brown => println!("brown"),
    }
}
