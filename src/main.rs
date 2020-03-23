#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Rust has many features to handle errors
// Sometimes error handling is required before compiling

// Rust errors are grouped into recoverable and unrecoverable errors
// recoverable errors are errors that report the problem and retry the operation
// unrecoverable errors are usually symptoms of bugs

// Rust doesn't have exceptions
// Rust instead has Result<T, E> (recoverable) and panic! (unrecoverable)

// When panic! macro executes program will print a failure message, unwind and cleanup the stack, then quit
// by default when a panic occurs the program starts unwinding (which means Rust walks back up the stack and cleans up the data from each function it encounters)
// alternative to unwinding is aborting which doesn't clean up anything
// memory used by the process will have to be cleaned up by the OS
// switching from unwinding to aborting will make a smaller binary result
fn main() {
    // panic!("crash and burn"); // example manual panic call

    // example panic call for backtrace
    // let v = vec![1, 2, 3];
    // v[99]; // indexing out of range (other languages have buffer over reads)

    // most errors are recoverable
    // result has two variant Ok(T) and Err(E)
    // T represents the type of value that will be returned in a success case with the Ok variant
    // E represents the type of error that will be returned in a failure case with the Err variant

    // Result<T, E> example
    let f = std::fs::File::open("hello.txt");
    // certain functions return a Result enum (open is one)
    // next you have to create a match expression to handle the result from the open
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // it's possible to match different errors by using .kinds() to match against ErrorKinds
            std::io::ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // cleaner code but come back after you know what unwrap_or_else does by using standard library documentation
    //let f = File::open("hello.txt").unwrap_or_else(|error| {
    //    if error.kind() == ErrorKind::NotFound {
    //        File::create("hello.txt").unwrap_or_else(|error| {
    //            panic!("Problem creating the file: {:?}", error);
    //        })
    //    } else {
    //        panic!("Problem opening the file: {:?}", error);
    //    }
    //});

    // unwrap method is a shortcut method that is implemented just like the match expression
    // if the result is the ok variant unwrap returns the value inside the ok
    // if the result is the err variant unwrap calls the panic! macro for us
    let f = std::fs::File::open("hello.txt").unwrap();

    // expect is similar to unwrap but it lets us choose the panic message
    let f = std::fs::File::open("hello.txt").expect("Failed to open hello.txt");
}

// when something is able to call something that might fails it may be better to handle the error from the calling code
// sending errors from inside the called function to the calling code is known as propagating the error and it gives more control to the calling code
// information on how to handle the error could be in the calling code
use std::fs::File;
use std::io;
use std::io::Read;

// returns the result with the variant
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// returns the result with the variant shortcut method using ? operator

fn read_username_from_file2() -> Result<String, io::Error> {
    // if the result is an ok variant the value inside the Ok will get returned from the expression and the program continues
    let mut f = File::open("hello.txt")?;
    // if the result is an err variant the err will be returned from the whole function
    // error values that have the ? operator called on them go through the from function which is used to convert errors from one type into another
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) // returns a Ok variant with the value inside it (even though the value was already taken out in the ?)

    // ? can only be used in functions that return result
}

// if you can ensure by manually inspecting the code that you'll never have an Err variant then it's acceptable to use unwrap

// have your code panic when it's possible that your code could end up in a bad state
// the bad state is not something that's expect to happen occasionally
// your code after this point needs to rely on not being in this bad state
// there's not a good wat to encode this information in the types you use

// if someone calls your code and passes values that don't make sense then it might be best to call panic! and alert them that they are using your library incorrectly
// similarly panic! is often appropriate if you're calling external code that is out of your control and it returns an invalid state that you have no way of fixing

// if failure is expected then it is more appropriate to return a Result rather than to make a panic! call

// when code performs operations on values code should verify the values are valid first and panic if the values aren't valid
// instead of having error checks in all your functions you could use Rust's type system
// For example if you have a type rather than an Option your program expect to have something rather than nothing
// your code then doesn't have to handle Sone and None variants only has to handle Some variant
// code that tries to pass nothing won't compile

// tedious validation version
// adds i32 to allow for negative values (for guessing_game project)
// let guess: i32 = match guess.trim().parse() {
//     Ok(num) => num,
//     Err(_) => continue,
// };
// checks if number is between 1 and 100
// if guess < 1 || guess > 100 {
//     println!("The secret number will be between 1 and 100.");
//     continue;
// }
// continues as usual
// match guess.cmp(&secret_number) {

// Creating Custom types for validation
// pub struct Guess {
//     value: i32, // creates a structure with a value allowed called i32
// }

// adds an associated function named new on Guess that creates instances of guess values
// the new function takes one parameter named value of type i32 and returns a guess struct
// the new functions tests value to make sure it's between 1 and 100 and if it doesn't pass it calls panic!
// if it does then we create a new guess with its value set to the value parameter and return the Guess
// next we have a method named value that borrows self
// this is called a getter because it's purpose is to get some data from its fields and return it
// this is important because the value field of guess struct is private
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }

//         Guess {
//             value
//         }
//     }

//     pub fn value(&self) -> i32 {
//         self.value
//     }
// }
