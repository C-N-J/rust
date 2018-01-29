fn main() {
    // VARIABLES
    // variables are immutable by default but can be made mutable with 'mut'.
    let mut x = 5;
    println!("The value of x = {}", x);
    x = 6;
    println!("The value of x = {}", x);

    // CONSTANTS
    // Constants are always immutable.
    // Use 'const' instead of the 'let' keyword and always define the type.
    // Can only be set to a constant expression and not the result of a function or anything that can only be computed at runtime.
    const MAX_POINTS: u32 = 100_000;

    // SHADOWING
    // Declare a new variable with the same name as a previous variable and the new variable shadows the previous variable
    // Do this by using 'let' when assigning the same variable name.

    let a = 5;

    let a = a + 1;

    let a = a + 1;

    println!("a = {}", a);

    // This is different to usng a mutable variable because the value is still immutable unless we explicitly use 'let'.
    // It also allows changing the type since we're essentially creating a new variable with the same name. For example:

    let spaces = "    ";
    let spaces = spaces.len();

    println!("There are {} spaces.", spaces);

    // 'spaces' is "converted" from a string to an int.
}
