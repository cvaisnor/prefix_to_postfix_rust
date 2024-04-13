use crate::stack_class; // import the stack_class module

// function to convert prefix to postfix
pub fn prefix_to_postfix(prefix_expression: &str) -> String {
    let mut stack = stack_class::Stack::new(100); // create a new stack

    // iterate through the prefix expression in reverse
    for i in (0..prefix_expression.len()).rev() {
        let c = prefix_expression.chars().nth(i).unwrap(); // get the character at index i

        // if the character is an operand
        if c.is_alphabetic() {
            stack.push(c.to_string()); // push the operand to the stack
        } else {
            // if the character is an operator
            let op1 = stack.pop(); // pop the first operand
            let op2 = stack.pop(); // pop the second operand
            let temp = op1.to_string() + &op2.to_string() + &c.to_string(); // create a temporary string
            stack.push(temp); // push the temporary string to the stack
        }
    }
    let postfix_expression = stack.pop(); // pop the postfix expression from the stack
    postfix_expression // return the postfix expression
}

pub fn prefix_to_infix(prefix_expression: &str) -> String {
    let mut stack = stack_class::Stack::new(100); // create a new stack

    // iterate through the prefix expression in reverse
    for i in (0..prefix_expression.len()).rev() {
        let c = prefix_expression.chars().nth(i).unwrap(); // get the character at index i

        // if the character is an operand
        if c.is_alphabetic() {
            stack.push(c.to_string()); // push the operand to the stack
        } else {
            // if the character is an operator
            let op1 = stack.pop(); // pop the first operand
            let op2 = stack.pop(); // pop the second operand
            let temp = "(".to_owned() + &op1.to_string() + &c.to_string() + &op2.to_string() + ")"; // create a temporary string
            stack.push(temp); // push the temporary string to the stack
        }
    }
    let infix_expression = stack.pop(); // pop the infix expression from the stack
    infix_expression // return the infix expression
}