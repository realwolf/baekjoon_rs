use std::{io};

fn main()
{
    let mut user_input_1 = String::new();
    let mut user_input_2 = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input_1);
    let _result = stdin.read_line(&mut user_input_2);

    //println!("input {} ", &user_input);
    let input_1 = user_input_1.replace("\r", "").replace("\n", "");
    let input_2 = user_input_2.replace("\r", "").replace("\n", "");

    if let (Ok(v1), Ok(v2)) = (input_1.parse::<i32>(), input_2.parse::<i32>())
    {
        if v1 > 0
        {
            if v2 > 0
            {
                println!("1");
            }
            else
            {
                println!("4");
            }
        }
        else
        {
            if v2 > 0
            {
                println!("2");
            }
            else
            {
                println!("3");
            }
        }
    }
}