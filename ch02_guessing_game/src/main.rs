use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    //print and get an input
    println!("Guess the number!");

    'game: loop {
        //display the difficulty level
        let max: u32 = loop {
            println!("Difficulty level: Easy (1-50)");
            println!("Difficulty level: Medium (1-100)");
            println!("Difficulty level: Hard (1-1000)");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            match choice.trim() {
                "1" => break 50,
                "2" => break 100,
                "3" => break 1000,
                _ => continue,
            }
        };

        let secret_number = rand::thread_rng().gen_range(1..=max);
        //debug
        //println!("The secret number is {secret_number}");

        let mut attempts = 0;
        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            println!("You guessed: {guess}");

            //let guess: u32 = guess.trim().parse().expect("Please type a number");

            //use match to handle the error
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            attempts += 1;

            if attempts > 10 {
                println!("You have tried 10 times, you lose! The secret number is {secret_number}");
                break;
            }

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    println!("You guessed {attempts} times");
                    break;
                }
            }
        }

        //ask if the user wants to play again?
        loop {
            println!("Do you want to play again? (y/n)");
            let mut play_again = String::new();
            io::stdin()
                .read_line(&mut play_again)
                .expect("Failed to read line");

            match play_again.trim() {
                "y" => continue 'game,
                "n" => break 'game,
                _ => continue,
            }
        }
    }
}
