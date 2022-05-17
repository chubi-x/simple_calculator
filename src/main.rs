// #######################
//      ALGORITHM
// #######################
// loop that runs until the user enters stop
// use  match to check user input
// function prompts user for an input "Do you want to use the calculator" Y/N statement
/* based on the above input, prompt the user to enter in the numbers he wants to operate on 
    an operation */
// run the appropriate operation on the provided inputs
// print results of operation
// ask user if he still wants to operate.

// #############################################
// PROGRAM
// #############################################

//macro to help read standard input
use text_io::read;
// class` to handle user input error
use std::num::ParseFloatError;

// main function
fn main() {
    println!("Welcome to the calculator! Type in Y to start and N when you're done calculating");
    // start indefinite loop until user stops it
    loop {
        // take user input (handle errors in case user enters wrong input)
        let mut line: String = read!();
        // make sure user input is eiher Y or N
        match line.to_lowercase().as_str() {
            "y" | "n" => {
                // check if user wants to compute
                while line.to_ascii_lowercase() != "n" {
                    // ask user for first number to operate on
                    println!("Now enter two numbers you want to operate on");
                    // collect first number
                    let num1 = read_num();
                    // ask user for second number
                    println!("Now for the second");
                    // take second number
                    let num2 = read_num();
                    // ask user for operation to perform
                    println!(
                        "What operation do you want to perform? \n
                        Options are 'add', 'sub', 'mult', and 'div'"
                    );
                    // take operation string from user
                    let operation: String = read!();
                    loop {
                        match operation.as_str() {
                            "add" | "sub" | "mult" | "div" => break,
                            _ => println!(
                                "invalid operation. \n
                            Options are 'add', 'sub', 'mult', and 'div' "
                            ),
                        }
                    }
                    // call function
                    calculator(num1, num2, &operation);
                    // ask user if they want to continue
                    println!("Do you still want to continue? Y/N");
                    // collect user input to determine if they continue or not
                    line = read!();
                }
                println!("Calculator ended.");
                break;
            }
            _ => {
                println!("Invalid user input\n Make sure you enter either Y or N");
            }
        };
    }
}
// calculator function
fn calculator(num1: f64, num2: f64, operation: &String) {
    let mut res = 0.0;
    if operation == "add" {
        res = (num1 + num2) as f64
    } else if operation == "sub" {
        res = num1 - num2;
    } else if operation == "div" {
        res = num1 / num2
    } else if operation == "mult" {
        res = num1 * num2
    } else {
        println!("Invalid operation");
    }
    println!("Result is {res}");
}
// function to read user number input
fn read_num() -> f64 {
    let mut num;
    loop {
        // take number as a string
        num = read!();
        // convert it to a float
        let num = convert_to_float(num);
        // hanndle the float conversion error
        match num {
            Ok(val) => return val,
            Err(ref val) => println!("Error! Invalid input: {:?}\n Try entering a number", val),
        };
    }
}
// funnction to convert user input to a float
fn convert_to_float(num: String) -> Result<f64, ParseFloatError> {
    let _num = num.parse::<f64>();
    return _num;
}
