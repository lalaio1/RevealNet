use colored::*;

pub fn alert(msg: &str) {
    println!("[{}] {}", "!".red().bold(), msg.white());
}
pub fn info(msg: &str) {
    println!("[{}] {}", "i".blue().bold(), msg.white());
}
pub fn starred(msg: &str) {
    println!("[{}] {}", "*".green().bold(), msg.white());
}
pub fn question(msg: &str) {
    println!("[{}] {}", "?".yellow().bold(), msg.white());
}
