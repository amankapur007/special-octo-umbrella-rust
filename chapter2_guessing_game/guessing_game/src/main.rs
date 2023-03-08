use std::io;

use rand::Rng;
fn main(){
    
    let secret_number = rand::thread_rng().gen_range(1..=100);




    loop {
        let mut guess_number = String::new();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess_number)
        .expect("Failed to read line");
    
        let guess_number:u32= match guess_number.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("You guessed: {guess_number}");

        match guess_number.cmp(&secret_number) {
            std::cmp::Ordering::Greater=> println!("Guess is big!"),
            std::cmp::Ordering::Less => println!("Guess is small!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
