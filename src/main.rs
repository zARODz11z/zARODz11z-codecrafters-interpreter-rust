use std::env;
use std::fs;
use std::io::{self, Write};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }
    let command = &args[1];
    let filename = &args[2];
    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            tokenize(&file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    fn tokenize(input: &str) {
        let mut line: i32 = 1;
        let mut errored: i8 = 0;
        let mut chars = input.chars().peekable();
    
        while let Some(&char) = chars.peek() {  // Peek at the next character
            match char {
                '(' => {
                    println!("LEFT_PAREN ( null");
                    chars.next();  // Move to the next character
                }
                ')' => {
                    println!("RIGHT_PAREN ) null");
                    chars.next();
                }
                '{' => {
                    println!("LEFT_BRACE {{ null");
                    chars.next();
                }
                '}' => {
                    println!("RIGHT_BRACE }} null");
                    chars.next();
                }
                ',' => {
                    println!("COMMA , null");
                    chars.next();
                }
                '.' => {
                    println!("DOT . null");
                    chars.next();
                }
                '-' => {
                    println!("MINUS - null");
                    chars.next();
                }
                '+' => {
                    println!("PLUS + null");
                    chars.next();
                }
                ';' => {
                    println!("SEMICOLON ; null");
                    chars.next();
                }
                '*' => {
                    println!("STAR * null");
                    chars.next();
                }
                '=' => {
                    chars.next();  // Consume the '=' character
                    // Check if the next character is also '='
                    if chars.peek() == Some(&'=') {
                        println!("EQUAL_EQUAL == null");
                        chars.next();  // Consume the second '=' character
                    } else {
                        println!("EQUAL = null");
                    }
                }
                '!' => {
                    chars.next(); //Consume the '!' char
                    // Check if the next character is '='
                    if chars.peek() == Some(&'=') {
                        println!("BANG_EQUAL != null");
                        chars.next(); // Consume the = in case of '!='
                    } else {
                        println!("BANG ! null");
                    }
                }
                '<' => {
                    chars.next(); //consume the <
                    if chars.peek() == Some(&'=') {
                        println!("LESS_EQUAL <= null");
                        chars.next();
                    } else {
                        println!("LESS < null");
                    }
                },
                '>' => {
                    chars.next(); //consume the >
                    if chars.peek() == Some(&'=') {
                        println!("GREATER_EQUAL >= null");
                        chars.next();
                    } else {
                        println!("GREATER > null");
                    }
                },
                '/' => {
                    chars.next(); //consume the /
                    if chars.peek() == Some(&'/') {
                        break;
                    } else {
                        println!("SLASH / null");
                    }
                },
                '\n' => {
                    line += 1;
                    chars.next();  // Move to the next line
                }
                _ => {
                    writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line, char).unwrap();
                    errored = 1;
                    chars.next();  // Move past the unexpected character
                }
            }
        }
        println!("EOF  null");
        if errored == 1 {
            std::process::exit(65);
        }
    }
}
