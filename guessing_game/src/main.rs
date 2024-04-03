use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); //variables are immutable by default so as to make it mutable we add mut in front of them
        io::stdin()
            .read_line(&mut guess) //& is a reference, it allows multiple parts of your code to access one piece of data without needing to copy that data into memory multiple times and by default they are immutable
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.eq_ignore_ascii_case("quit") {
            println!("You quit the game");
            break;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
