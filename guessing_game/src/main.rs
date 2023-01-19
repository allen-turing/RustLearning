use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {
    println!("----Welcome to Number Guessing!----");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop
    {
        println!("💡 Guess the number 💡");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the number!");
        
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("**Please enter a valid number**");
                continue;
            } 
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => 
            {
                println!("You Win the game!");
                break;
            }
        }
    }
    

    

   
}
