use std::io;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input);

    let input = user_input.replace("\r", "").replace("\n", "");
    let split = input.split(" ");
    let vec = split.collect::<Vec<&str>>();

    if let (Ok(a), Ok(b), Ok(c)) = (
        vec[0].parse::<i32>(),
        vec[1].parse::<i32>(),
        vec[2].parse::<i32>()
    ) {
        println!("{}", (a + b) % c);
        println!("{}", ((a % c) + (b % c)) % c);
        println!("{}", (a * b) % c);
        println!("{}", ((a % c) * (b % c)) % c);
    }
}
