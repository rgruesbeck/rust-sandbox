// #[allow(unused_variables)]
// #[allow(unused_assignments)]

fn primitives() {
    // bools
    // explicit let som_bool: bool = true;
    let mut some_bool = true; // or false
    println!("mutable bool: {}", some_bool);

    some_bool = false;
    println!("mutable bool: {}", some_bool);

    // signed 8 bit numbers
    let signed_eight_bit_num: i8 = 10; // from -128 to +127
    println!("i8 number: {}", signed_eight_bit_num);

    println!("Min i8 is {}", std::i8::MIN);
    println!("Max i8 is {}", std::i8::MAX);

    // rust decides to crash before it creates bad data
    // take this overflow as an example
    let test_num = signed_eight_bit_num + 120; // 130 is greater than then max
    println!("i8 overflow 10 + 130: {}", test_num); // THIS WILL PANIC, run with "cargo run --release" to see value

    // unsigned 8 bit numbers
    let unsigned_eight_bit_num: u8 = 10; // from 0 to 225
    println!("i8 number: {}", unsigned_eight_bit_num);

    // integer types are i and u
    // can be 8, 16, 32, 64, 128 bit size
    let signed_big_num: i128 = 10; // from -2^128
    println!("i128 number: {}", signed_big_num);
    println!("Min i128 is {}", std::i128::MIN);
    println!("Max i128 is {}", std::i128::MAX);

    // if no integer type the default is i32
    let default_num = 10;
    println!("default number type {}", default_num);

    // can also use isize which will default to either 32 or 64 bit
    // https://doc.rust-lang.org/std/primitive.isize.html
    let sized_num: isize = 10;
    println!("isized int: {}", sized_num);

    // floats!!
    // defaults to f64
    let float_num: f32 = 10.0;
    println!("32f float: {}", float_num);

    // chars
    // https://doc.rust-lang.org/std/char/index.html
    let chr: char = 'a'; // more than just ascii!
    println!("char: {}", chr);
}

fn strings() {
    // https://www.youtube.com/watch?v=ClPrjjHmo2Y&list=PLLqEtX6ql2EyPAZ1M2_C0GgVd4A-_L4_5&index=6

    // two types of strings
    // both are groupings of chars u8's

    // string slice
    // immutable (with exceptions)
    // allocated on the stack (usually)
    // sometimes a heap reference
    // sometimes embedded in the code
    let string_slice: &str = "Howdy";
    println!("string slice: {}", string_slice);

    // String
    // mutable
    // allocated on the heap
    let string_string: String = String::from("Partner");
    println!("string string: {}", string_string);


    // slices -> String
    let string_from_str: String = string_slice.to_string();
    println!("string from string slice: {}", string_from_str);

    // literal -> String
    let string_from_literal: String = "Hardcoded literal".to_string();
    println!("string from string literal: {}", string_from_literal);

    // String -> slice 
    let slice_from_string: &str = &string_from_str;
    println!("slice from string: {}", slice_from_string);

    // Adding strings

    // the + opperator cannot concat string slices or literals
    // creates compile error: `+` cannot be used to concatenate two `&str` strings
    // let third = "first" + "second";

    // Using array.concat()
    let concat_string_literals_with_array = ["first", "second"].concat();
    println!("concat string literals with array.concat() {}", concat_string_literals_with_array);

    // Using format!()
    let concat_string_literal_with_format_macro = format!("{} {}", "first", "second");
    println!("concat string literals with format! {}", concat_string_literal_with_format_macro);

    // Using String + <literal/slice>
    let string_plus = string_string + " there!";
    println!("concat string literals with String +: {}", string_plus);

    // First string must be the owner
    let string_more = String::from("!");
    let string_plus_plus = string_plus + &string_more;
    println!("concat string literals with String +: {}", string_plus_plus);

    // Use push_str
    let mut mut_string = String::new();
    println!("new mut string: {}", mut_string);

    // Push string slice
    mut_string.push_str(string_slice);
    println!("push slice: {}", mut_string);

    // Push string literal
    mut_string.push_str(" Partner!");
    println!("push literal: {}", mut_string);

    // Push char (char literal must be single qoutes)
    mut_string.push('!'); // "!" would be a &str
    println!("push char: {}", mut_string);


    // Substrings
    let string_from_substring: &str = &string_slice[0..3]; // start at zero, go upto but dont include 3. similar to [0..3)
    println!("substring [0..2] {}", string_from_substring);
    
    // note: over indexing [0..200] will PANIC
    // a safer way is to use string.chars().nth(1)
}

// functions and procedures 
// functions return a value, procedures do not
fn some_function(param_a: f32, param_b: i128) -> f32 {
    10.0
}


fn main() {
    primitives();
    strings();
}