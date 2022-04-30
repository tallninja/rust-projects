fn calculate_square(num: i32) -> i32 {
    num * num
}

fn main() {
    let num = 20;
    let square = calculate_square(num);
    println!("The square of {:?} is {:?}", num, square);
}
