use rand::Rng;
use std::io;

fn main() {
    let min = 0;
    let max = 100;
    let mut attempt = 1;
    let x: i32 = get_random_number(Some(min), Some(max));

    println!("Guess the number between {} and {}.", min, max);

    loop {
        println!("What is your guess?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("invalid input");
        let res = parse_string(&input);

        if res == x {
            println!("Nice! You found it with {} guesses!", attempt);
            break;
        }

        match res {
            res if res > x => print!("Too high, "),
            res if res < x => print!("Too low, "),
            _ => {}
        }

        attempt += 1;
    }
}

fn parse_string(str: &str) -> i32 {
    str.trim().parse().unwrap()
}

fn get_random_number(min: Option<i32>, max: Option<i32>) -> i32 {
    rand::thread_rng().gen_range(min.unwrap_or(0)..max.unwrap_or(100))
}
