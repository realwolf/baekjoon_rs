use std::{io};

fn main()
{
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input);

    let input = user_input.replace("\r", "").replace("\n", "");
    println!("{}??!",  input);
}