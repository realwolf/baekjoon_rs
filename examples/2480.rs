use std::{io, cmp};

fn main()
{
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input);

    let input = user_input.replace("\r", "").replace("\n", "");
    let split = input.split(" ");
    let vec = split.collect::<Vec<&str>>();

    if let (Ok(a), Ok(b), Ok(c)) = (vec[0].parse::<i32>(), vec[1].parse::<i32>(), vec[2].parse::<i32>())
    {
        if a == b && a == c
        {
            println!("{}", 10000 + a * 1000);
        }
        else if a == b || a == c{
            println!("{}", 1000 + a * 100);
        }
        else if b == c {
            println!("{}", 1000 + b * 100);
        }
        else
        {
            println!("{}", cmp::max(a, cmp::max(b, c)) * 100);
        }
    }
}