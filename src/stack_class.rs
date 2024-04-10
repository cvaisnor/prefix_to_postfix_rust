// this holds the stack class implementation

pub struct Stack {
    top: i32, // top of the stack
    capacity: i32, // capacity of the stack
    array: Vec<String>, // array to store elements
}

impl Stack {
    // create a new stack with capacity n
    pub fn new(n: i32) -> Stack {
        Stack {
            top: -1, // top is -1
            capacity: n, // capacity is n
            array: Vec::new(), // create an empty vector
        }
    }

    // check if the stack is full
    pub fn is_full(&self) -> bool {
        self.top == self.capacity - 1
    }

    // check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.top == -1
    }

    // push an element to the stack
    pub fn push(&mut self, item: String) {
        if self.is_full() {
            println!("Stack Overflow");
            return;
        }
        self.top += 1;
        self.array.push(item);
    }

    // pop an element from the stack
    pub fn pop(&mut self) -> String {
        if self.is_empty() {
            println!("Stack Underflow");
            return String::from("");
        }
        let item = self.array.pop().unwrap();
        self.top -= 1;
        item
    }

    // get the top element of the stack
    // pub fn peek(&self) -> String {
    //     if self.is_empty() {
    //         return String::from("");
    //     }
    //     self.array[self.top as usize].clone()
    // }
}