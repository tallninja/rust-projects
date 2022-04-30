fn main() {
    let num: i32 = 200;

    let num_gt_100: bool = num > 100;
    match num_gt_100 {
        true => println!(">100"),
        false => println!("<=100"),
    }
}
