use std::io;

fn input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    return Ok(buffer.trim().to_owned());
}

fn main() {
    let mut words: Vec<String> = vec![];
    let mut count = 0;

    while count < 2 {
        match input() {
            Ok(word) => {
                words.push(word);
                count += 1;
            }
            Err(e) => println!("error: {e}"),
        };
    }

    for word in words {
        println!("original: {word}, uppercase: {:?}", word.to_uppercase());
    }
}
