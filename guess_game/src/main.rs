use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {

    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("can not read the input");

        let guess_number:i32 = guess_number.trim().parse().expect("Please type a number");

        println!("Your guess number is {}", guess_number);

        //let guess_number = guess_number.to_string();
        // match guess_number.trim().parse::<i32>(){
        //     Ok(guess_number) =>println!("transfer successful! {}", guess_number),
        //     Err(e) => println!("transfer field! {}", e),
        // }

        // if guess_number > secret_number {
        //     println!("too big");
        // } else if guess_number < secret_number {
        //     println!("too small");
        // } else {
        //     println!("you win");
        //     break;
        // }

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            
        }  
    }
    
}
