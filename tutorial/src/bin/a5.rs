fn main() {
    let mut counter: i32 = 1;
    loop {
        println!("{counter}");
        if counter >= 4 {
            break;
        }
        counter += 1;
    }
    println!("Done !");
}
