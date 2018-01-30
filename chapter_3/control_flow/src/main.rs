fn main() {
    let number = 3;

    if number < 5 {
        println!("True");
    }
    else {
        println!("False");
    };

    // Using if in a let statement
    let condition = true;
    let number = if condition {
        5
    }
    else {
        6
    };

    println!("{}", number);
}
