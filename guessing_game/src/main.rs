extern crate

use std::io;

fn main() {
    println!("Guess the number!"); // Ending in a ! means its a macro not a method

    println!("Please input your guess.");

    let mut guess = String::new();  // the :: means we're calling a class(static) method

    io::stdin().read_line(&mut guess) // & is a reference to data (seems like a pointer). They are immutable by default so we write `&mut guess` to make it mutable rather than `&guess`
        .expect("Failed to read line"); // Calling #read_line returns a io::Result type which we can call #expect on. io::Result#expect will print the input if it errors or the value of the OK result if its OK

    println!("You guessed {}", guess);
}
