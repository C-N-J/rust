fn main() {
    // Datatypes are split into two subsets: scalar and compound.

    // Scalar Types
    // 4 primary scalar types: int, floats, bools and chars.

    // Ints

    let decimal = 98_222;
    let hex     = 0xff;
    let octal   = 0o77;
    let binary  = 0b1111_0000;
    let byte    = b'A';

    // Floats
    let float_1      = 2.0;
    let 32_bit_float: f32 = 3.0;

    // Bools
    let t = true;
    let f: bool = false;

    // Chars
    let char_1 = 'z';
    let char_2 = 'Z';

    // Compound Types
    // 2 compound types: tuples and arrays.

    // Tuples //

    // Access either using pattern matching or a dot

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Access either using pattern matching or a dot
    let (x, y, z) = tup;
    println!("The value of y = {}", y);

    let tup_2 (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup_2.0;

    let six_point_four = tup_2.1;

    let one = tup_2.2;

    // Arrays //
    // Arrays can only contain the same data type and are a fixed length.
    // Vectors are more flexible.
    // Use arrays when you know the length of the data such as a list of the months of the year since it can only have a length of 12.
    let array = [1, 2, 3, 4, 5];

    let first_array_element = array[0];

    let second_array_element = array[1];
}
