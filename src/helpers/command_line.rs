use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

// Get User Request
pub fn get_user_request(prompt: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a Specific Color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", prompt);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read User Input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response!");

    // Trim whitespace and return
    return user_response.trim().to_string();
}