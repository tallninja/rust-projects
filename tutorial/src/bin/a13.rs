fn main() {
    let my_numbers = vec![10, 20, 30, 40];

    for &number in &my_numbers {
        if number == 30 {
            println!("thirty");
        } else {
            println!("{number}");
        }
    }

    println!("{:?}", my_numbers.len());
}
