extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!( "Guess the number!" );
    println!( "-===============-" );

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess_count = 1;
    
    loop
    {
        println!("Input your guess [{}]", guess_count);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Invalid input: {}", error );
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win! Attempts: {}", guess_count);
                break;
            }
        }

        guess_count += 1;
    }
}
