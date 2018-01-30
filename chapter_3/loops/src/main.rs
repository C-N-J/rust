fn main() {
    // Loop using a break
    let mut loop_count = 1;
    loop {
        println!("{}", loop_count);

        if loop_count == 100 {
            break;
        }

        loop_count += 1;
    }

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For loop with an array

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}", element);
    }

    // For loop a certain number of times
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
