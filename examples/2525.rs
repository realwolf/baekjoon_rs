use std::io;

fn main() {
    let mut user_input_1 = String::new();
    let mut user_input_2 = String::new();
    let stdin = io::stdin();
    let _result = stdin.read_line(&mut user_input_1);
    let _result = stdin.read_line(&mut user_input_2);

    let input_1 = user_input_1.replace("\r", "").replace("\n", "");
    let input_2 = user_input_2.replace("\r", "").replace("\n", "");
    let split = input_1.split(" ");
    let vec = split.collect::<Vec<&str>>();

    if let (Ok(a), Ok(b), Ok(c)) = (
        vec[0].parse::<i32>(),
        vec[1].parse::<i32>(),
        input_2.parse::<i32>())
    {
        let mut h = a;
        let mut m = b + c;

        loop
        {
            if m >= 60
            {
                h = h + 1;
                m = m - 60;
            }
            else
            {
                break;
            }
        }

        println!("{} {}", h % 24, m % 60);
    }
}
