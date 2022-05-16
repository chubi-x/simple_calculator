// #######################
//      ALGORITHM
// #######################
// loop that runs until the user enters stop
// use  match to check user input
// function prompts user for an input "Do you want to use the calculator" Y/N statement
// based on the above input, prompt the user to enter in an operation and the numbers he wants to operate on
// run the appropriate operation on the provided inputs
// print results of operation
// ask user if he still wants to operate.
use text_io::read;

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
                    // take first number
                    let num1: f64 = read!();
                    // ask user for second number
                    println!("Now for the second");
                    // take second number
                    let num2: f64 = read!();
                    // ask user for operation to perform
                    println!(
                        "What operation do you want to perform? \n
                Options are 'add', 'sub', 'mult', and 'div'
            "
                    );
                    // take operation string from user
                    let operation: String = read!();
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
    println!("Result is {}", res);
}
