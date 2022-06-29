// extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
// for comparing the numbers and ordering is used to 
//The Ordering type is another enum and has the variants Less, Greater, and Equal. 
// These are the three outcomes that are possible when you compare two values.
use std::io;
//input/output library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // we create a mutable variable guess of string type


        // we can als use std::io::stdin even if we used the 1st line std::io
        // he stdin function returns an instance of std::io::Stdin, 
        // which is a type that represents a handle to the standard input for your terminal
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // take whatever the user types into standard input and append that into a string 
        // without overwriting its contents 
        // so we therefore pass that string as an argument. 
        // The string argument needs to be mutable so the method can change the string’s content.
        // & indicates that this argument is a reference
        // multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. 

        // TRIM
        // The trim method on a String instance will eliminate any whitespace at the beginning and end
            
        // PARSE
        // The parse method on strings parses a string into some kind of number. 
        // Because this method can parse a variety of number types, 
        // we need to tell Rust the exact number type we want by using let guess: u32. 
        // The colon (:) after guess tells Rust we’ll annotate the variable’s type. 
        // Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer.     
            // If parse is not able to turn the string into a number, it will return an Err value 
            // that contains more information about the error. 
            // The Err value does not match the Ok(num) pattern in the first match arm, 
            // but it does match the Err(_) pattern in the second arm. 
            // The underscore, _, is a catchall value       
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };



        // We use a match expression to decide what to do next based on 
        // which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    // we can als use std::io::stdin even if we used the 1st line std::io
    // he stdin function returns an instance of std::io::Stdin, 
    // which is a type

    // take same structure - 3 variables, take same 3 variables for enums
    // print the 3 struct variable sand enums too
    // we can't print 3 variables for enum at a time 