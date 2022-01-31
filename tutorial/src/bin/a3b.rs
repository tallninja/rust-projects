fn main() {
    let my_var: i32 = 3;

    if my_var > 5 {
        println!(">5");
    } else if my_var < 5 {
        println!("<5");
    } else {
        println!("=5");
    } else {
        println!("unknown");
    }
}
