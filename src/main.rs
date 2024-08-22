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
        let mut errored : i8 = 0;
        for char in input.chars() {
            match char {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                ',' => println!("COMMA , null"),
                '.' => println!("DOT . null"),
                '-' => println!("MINUS - null"),
                '+' => println!("PLUS + null"),
                ';' => println!("SEMICOLON ; null"),
                '*' => println!("STAR * null"),
                '\n'=> line += 1,
                _ => {
                    writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line, char).unwrap();
                    errored = 1;
                    },
            
            }
        }
        println!("EOF  null");
        if errored == 1{
            std::process::exit(65);
        }
    }
}

