
use std::io; //input/output
use std::cmp::Ordering;
use rand::Rng;//rand num gen
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");

        let mut guess= String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            // trim method eliminates \n or \r\n, resulting in just 5
            // parse method on strings converts a string to another type
            // .expect("Please type a number!") crashes program in the event of an error (input is non-u32)
            let guess: u32 = match guess.trim().parse()  {
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