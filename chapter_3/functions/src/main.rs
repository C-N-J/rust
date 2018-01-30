fn main() {
    println!("Hello, world!");

    let z = 7;

    another_function(5, 6, z);

    let a = a_function_that_returns(z);

    println!("{} multiplies by 2 is {}", z, a);
}

fn another_function(x: i32, y: i32, z: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);
}

fn a_function_that_returns(x: i32) -> i32 {
    // Rust returns the result of an expression (no ;) does not return the result of statements
    x * 2
}
