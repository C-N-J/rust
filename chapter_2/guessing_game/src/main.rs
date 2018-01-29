extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); // Ending in a ! means its a macro not a method

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // the :: means we're calling a class(static) method

        io::stdin().read_line(&mut guess) // & is a reference to data (seems like a pointer).They are immutable by default so we write `&mut guess` to make it mutable rather than `&guess`
            .expect("Failed to read line"); // Calling #read_line returns a io::Result type which we can call #expect on. io::Result#expect will print the input if it errors or the value of the OK result if its OK

        // We 'shadow' the guess variable so we can change it's type rather than having to create two variable like 'int_guess' and 'str_guess'.
        // The type cannot be infered as it was when we originally declared 'guess' as a string since there are multiple types of number e.g. i32, u32, i64. This means we need to declare the type.
        // #trim removes any whitespace and the new line from the string 'guess'. #parse on strings parses it into a number and returns a 'Result' like #read_line.
        // Use a #match to add handling for the returned Result enum (line 34 for #match notes).
        // If guess is converted successfully it will return 'Ok' with a value which we return else it will return an 'Err'.
        // The '_' in 'Err' is a catchall value so 'Err(_)' catches all errors no matter what. 'Continue' will go to the next iteration of the loop, same as rubys #next.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };

        println!("You guessed {}", guess);

        // #cmp compares two values and returns a variant of the Ordering enum.
        // #match is made up of 'arms'. An 'arm' consists of a pattern. #match takes a value and compares it against each 'arm'.
        // In this instance #cmp will return an Ordering enum such as 'Ordering::Greater' which is passed to #match. #match then compares it against each arm and runs the correct code.
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
  }
}
