use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
       println!("Please input your guess.");
        let mut guess = String::new();


        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

        let guess:  u32 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
        } 
    }
}
