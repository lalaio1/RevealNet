use colored::*;
use std::io::{self, Write};

pub fn print_banner() {
    let raw_banner = r#"
<.red> _____                 _ <.white>_____     _   
<.red>| __  |___ _ _ ___ ___| |<.white>   | |___| |_ 
<.red>|    -| -_| | | -_| .'| |<.white> | | | -_|  _|
<.red>|__|__|___|\_/|___|__,|_|<.white>_|___|___|_|     [ <.red> by:<.white> lalaio1 <.red> v:<.white> 1.0 ]
"#;

    let mut current_color = Color::White;
    let mut buffer = String::new();
    let mut chars = raw_banner.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' && chars.peek() == Some(&'.') {
            if !buffer.is_empty() {
                print!("{}", buffer.color(current_color).bold());
                buffer.clear();
            }
            chars.next();
            let mut tag = String::new();
            while let Some(&nc) = chars.peek() {
                chars.next();
                if nc == '>' { break; }
                tag.push(nc);
            }
            current_color = match tag.as_str() {
                "red" => Color::Red,
                "white" => Color::White,
                _ => current_color,
            };
        } else {
            buffer.push(c);
        }
    }

    if !buffer.is_empty() {
        print!("{}", buffer.color(current_color).bold());
    }
    println!();

    io::stdout().flush().unwrap();
}