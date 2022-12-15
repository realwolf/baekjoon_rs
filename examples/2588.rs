use std::io;

fn main() {
    let mut user_input_a = String::new();
    let mut user_input_b = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input_a);
    let _result = stdin.read_line(&mut user_input_b);

    let input_a = &user_input_a[0..3];
    let input_b = &user_input_b[0..3];

    if let (Ok(a), Ok(b)) = (input_a.parse::<i32>(), input_b.parse::<i32>()) {
        println!("{}", a * (b % 10));
        println!("{}", a * (b / 10 % 10));
        println!("{}", a * (b / 100 % 10));
        println!("{}", a * b);
    }
}
