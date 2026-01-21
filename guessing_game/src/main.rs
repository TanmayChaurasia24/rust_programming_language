use rand::Rng;
use std::cmp::Ordering;
use std::io; // for taking input from the user

fn main() {
    println!("----------------------GUESSING GAME-------------------------");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Guess the number: ");
        let mut guess: String = String::new(); // creating a variable name guess of type string and making it muttable so that we can edit its value.
        io::stdin()
            .read_line(&mut guess) // giving mutable reference to read_line, so that It can also modify the guess variable
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //shadowing the previous value of guess and parsing it to u32 and expect is used to handle the error 

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
