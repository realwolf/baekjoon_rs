use std::io;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input);

    let input = user_input.replace("\r", "").replace("\n", "");
    let split = input.split(" ");
    let vec = split.collect::<Vec<&str>>();

    if let (Ok(a), Ok(b)) = (
        vec[0].parse::<i32>(),
        vec[1].parse::<i32>())
    {
        let mut h = a + 24;
        let m = b + 60 - 45;

        if m < 60
        {
            h = h - 1;
        }

        println!("{} {}", h % 24, m % 60);
    }
}
