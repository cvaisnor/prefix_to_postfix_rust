use conversion_functions::{prefix_to_infix,prefix_to_postfix};
use std::thread; // for running the conversion functions in parallel

mod stack_class;
mod conversion_functions;
// mod utils;

fn main() {

    println!("Prefix to Infix/Postfix Converter.\n");
    
    let prefix_expression: &str = "-+ABC"; // prefix expression

    println!("Creating threads for conversion functions...\n");
    // create a thread for each conversion function
    let thread1 = thread::spawn(move || prefix_to_infix(prefix_expression)); // convert prefix to infix
    let thread2 = thread::spawn(move || prefix_to_postfix(prefix_expression)); // convert prefix to postfix

    // wait for the threads to finish
    let infix_expression = thread1.join().unwrap(); // get the infix expression
    let postfix_expression = thread2.join().unwrap(); // get the postfix expression
    
    // print the results
    println!("The prefix expression is: {}", prefix_expression); // print the prefix expression
    println!("The infix expression is: {}", infix_expression); // print the infix expression
    println!("The postfix expression is: {}", postfix_expression); // print the postfix expression
}
