//Source
//https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use std::io;
//cmp: compares two values
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    //creating a loop
    loop{
        println!("Please input your guess.");

        //creating a mutateable string, (mut) allows for the string to be modified
        //Rust does not variably change variable types, it automatically set to a string
        let mut guess = String::new();
    
        //reading line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //Rust has three number types, 1-100: i32, 32-bit number: u32, and 64-bit number: i64
        //able to have two of the same variable names? Variable Shadowing.
        //parse converts a string to a number, trim eliminates whitespace within the input
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        //invalid input conversion
        let guess: u32 = match guess.trim().parse(){
            //if parse is able to return a value, it is stored as an "Ok" value.
            Ok(num) => num,
            //if not it will return an Err value. The underscore is a catchall value.
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
    
        //match creates the check || Guess.compare &Secret_Number
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            //breaks the loop on the correct guess
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }



}
