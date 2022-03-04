fn main() {
    let n: u32 = 11;
    println!("{n}th fibonacci number is: {}", get_nth_fib(n));
}

fn get_nth_fib(n: u32) -> u64 {
    let mut a: u64 = 1;
    let mut b: u64 = 1;
    let mut c: u64 = a + b;
    for _i in 0..n - 1 {
        a = b;
        b = c;
        c = a + b;
    }
    a
}
