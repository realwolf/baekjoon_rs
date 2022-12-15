use std::{io};

fn main()
{
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input);

    //println!("input {} ", &user_input);
    let input = user_input.replace("\r", "").replace("\n", "");

    if let Ok(v) = input.parse::<i32>()
    {
        if v % 4 == 0 && ( v % 100 != 0 || v % 400 == 0 )
        {
            println!("1");
        }
        else
        {
            println!("0");
        }
    }
}