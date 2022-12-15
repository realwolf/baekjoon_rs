use std::io;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input);

    let input = user_input.replace("\r", "").replace("\n", "");
    let split = input.split(" ");
    let vec = split.collect::<Vec<&str>>();

    if let (Ok(a), Ok(b), Ok(c), Ok(d), Ok(e), Ok(f)) = (
        vec[0].parse::<i32>(),
        vec[1].parse::<i32>(),
        vec[2].parse::<i32>(),
        vec[3].parse::<i32>(),
        vec[4].parse::<i32>(),
        vec[5].parse::<i32>(),
    ) {
        println!("{} {} {} {} {} {}", 1 - a, 1 - b, 2 - c, 2 - d, 2 - e, 8 - f);
    }
}