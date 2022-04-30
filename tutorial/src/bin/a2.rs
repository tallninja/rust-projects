fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn print_results(result: i32) {
    println!("Results: {result}");
}

fn main() {
    let result: i32 = sum(34, 45);
    print_results(result);
}
