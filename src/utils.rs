// holds utility functions

// read a line from the user
pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// read the lines in a file
pub fn read_file(file_name: &str) -> String {
    let contents = std::fs::read_to_string(file_name).unwrap();
    contents
}

