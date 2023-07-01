use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Secret is {secret}");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess : u32 = match guess.trim().parse(){
            Result::Err(_) => {
                println!("Please type a number!");
                continue;
            },
            Result::Ok(num) => num,
        };
        println!("You guessed {guess}");

        match guess.cmp(&secret){
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Less => println!("Too small!"),
        };
    }
}
