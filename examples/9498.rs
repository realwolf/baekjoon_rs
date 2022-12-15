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
        match v
        {
            v if v >= 90 => { println!("A"); }
            v if v >= 80 => { println!("B"); }
            v if v >= 70 => { println!("C"); }
            v if v >= 60 => { println!("D"); }
            _ => { println!("F"); }
        }
    }
}