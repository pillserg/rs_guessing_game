extern crate rand;
extern crate docopt;
extern crate rustc_serialize;

use std::io;
use std::cmp::Ordering;

use rand::Rng;
use docopt::Docopt;


fn guessing_game(min: u64, max: u64) {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(min, max);
    let mut count: u64 = 0;
    loop {
        println!("Please input your guess.\t attempt: {}", count);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");
        
        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number...");
                continue;
            }
        };
        count += 1;    
        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {} attempts! Congrats", count);
                break;
            }
        };

    }
}


fn guessing_bot(min: u64, max: u64) {
    let mut min = min;
    let mut max = max;
    let mut guess = max;
    let mut diff = Ordering::Less;
    let mut count: u64 = 0;

    println!("Conceive a number!");

    loop {
        guess = match diff {
            Ordering::Less => {
                max = guess;
                min + (max - min) / 2
            },
            Ordering::Greater => {
                min = guess;
                min + (max - min) / 2
            },
            _ => panic!("What?")
        };       
        println!("Your number is [ < | > | = ] then {}?", guess);
        
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        diff = match input.trim() {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            "=" => {
                println!("I won in {} rounds - yay", count);
                break
            },
            _ => {
                println!("wrong input");
                continue
            }
        };
        count += 1;
        println!("{:?}", diff);
        if min == max && diff != Ordering::Equal {
            println!("Cheater!!!");
            break
        }
    }
}


static USAGE: &'static str = "
Usage: guessing_game [options]

Options:
    -h, --help         Show this message.
    -b, --botmode      Let bot guess the number.
    --min <min>          Min num bound [default: 0]
    --max <max>          Max num bound [default: 1000]
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_botmode: bool,
    flag_min: u64,
    flag_max: u64
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    match args.flag_botmode {
        true => guessing_bot(args.flag_min, args.flag_max),
        false => guessing_game(args.flag_min, args.flag_max)
    }
}
