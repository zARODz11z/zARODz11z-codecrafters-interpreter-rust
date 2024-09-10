use std::env;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;

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
}

fn tokenize(input: &str) {
    let mut line: i32 = 1;
    let mut errored: i8 = 0;
    let mut chars = input.chars().peekable();

    while let Some(&char) = chars.peek() {
        match char {
            '(' | ')' | '{' | '}' | ',' | '.' | '-' | '+' | ';' | '*' => {
                print_simple_token(char);
                chars.next();
            }
            '=' | '!' | '<' | '>' => {
                print_complex_token(char, &mut chars);
            }
            '/' => {
                handle_slash(&mut chars);
            }
            '"' => {
                if let Err(_) = handle_string_literal(&mut chars, &mut line) {
                    errored = 1;
                }
            }
            '0'..='9' => {
                handle_number(&mut chars);
            }
            '\n' => {
                line += 1;
                chars.next();
            }
            ' ' | '\t' => {
                chars.next();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                handle_identifier(&mut chars);

            } _ => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line, char).unwrap();
                errored = 1;
                chars.next();
            }
        }
    }
    println!("EOF  null");
    if errored == 1 {
        std::process::exit(65);
    }
}

fn print_simple_token(char: char) {
    let token_type = match char {
        '(' => "LEFT_PAREN",
        ')' => "RIGHT_PAREN",
        '{' => "LEFT_BRACE",
        '}' => "RIGHT_BRACE",
        ',' => "COMMA",
        '.' => "DOT",
        '-' => "MINUS",
        '+' => "PLUS",
        ';' => "SEMICOLON",
        '*' => "STAR",
        _ => unreachable!(),
    };
    println!("{} {} null", token_type, char);
}

fn print_complex_token(char: char, chars: &mut std::iter::Peekable<std::str::Chars>) {
    match char {
        '=' => {
            chars.next(); // Consume the '=' char
            if chars.peek() == Some(&'=') {
                println!("EQUAL_EQUAL == null");
                chars.next(); // Consume the '=' in case of '=='
            } else {
                println!("EQUAL = null");
            }
        }
        '!' => {
            chars.next(); // Consume the '!' char
            if chars.peek() == Some(&'=') {
                println!("BANG_EQUAL != null");
                chars.next(); // Consume the '=' in case of '!='
            } else {
                println!("BANG ! null");
            }
        }
        '<' => {
            chars.next(); // Consume the '<' char
            if chars.peek() == Some(&'=') {
                println!("LESS_EQUAL <= null");
                chars.next(); // Consume the '=' in case of '<='
            } else {
                println!("LESS < null");
            }
        }
        '>' => {
            chars.next(); // Consume the '>' char
            if chars.peek() == Some(&'=') {
                println!("GREATER_EQUAL >= null");
                chars.next(); // Consume the '=' in case of '>='
            } else {
                println!("GREATER > null");
            }
        }
        _ => unreachable!(),
    };
}


fn handle_slash(chars: &mut std::iter::Peekable<std::str::Chars>) {
    chars.next();
    if chars.peek() == Some(&'/') {
        while let Some(&next_char) = chars.peek() {
            if next_char == '\n' {
                break;
            }
            chars.next();
        }
    } else {
        println!("SLASH / null");
    }
}

fn handle_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) {
    let mut identifier = String::new();
    let mut reserved_words = HashMap::new();
    reserved_words.insert("and", "AND");
    reserved_words.insert("class", "CLASS");
    reserved_words.insert("else", "ELSE");
    reserved_words.insert("false", "FALSE");
    reserved_words.insert("for", "FOR");
    reserved_words.insert("fun", "FUN");
    reserved_words.insert("if", "IF");
    reserved_words.insert("nil", "NIL");
    reserved_words.insert("or", "OR");
    reserved_words.insert("print", "PRINT");
    reserved_words.insert("return", "RETURN");
    reserved_words.insert("super", "SUPER");
    reserved_words.insert("this", "THIS");
    reserved_words.insert("true", "TRUE");
    reserved_words.insert("var", "VAR");
    reserved_words.insert("while", "WHILE");

    // Consume the characters before a space ' '
    while let Some(&char) = chars.peek() {
        if char.is_alphanumeric() || char == '_' {
            identifier.push(char);
            chars.next();
        } else {
            break;
        }
    }

    // Check if the identifier is a reserved word
    if let Some(token_type) = reserved_words.get(identifier.as_str()) {
        println!("{} {} null", token_type, identifier);
    } else {
        println!("IDENTIFIER {} null", identifier);
    }
}

fn handle_number(chars: &mut std::iter::Peekable<std::str::Chars>) {
    let mut number = String::new();

    // Consume the digits before the decimal point
    while let Some(&char) = chars.peek() {
        if char.is_digit(10) {
            number.push(char);
            chars.next();
        } else {
            break;
        }
    }

    // Check for a decimal point followed by more digits
    if let Some(&'.') = chars.peek() {
        number.push('.');
        chars.next(); // Consume the decimal point

        while let Some(&char) = chars.peek() {
            if char.is_digit(10) {
                number.push(char);
                chars.next();
            } else {
                break;
            }
        }
    }
    
    // Parse the number as a floating-point (f64) regardless of whether it's an integer or float
    let literal_value: f64 = number.parse().unwrap();

    // Print the number, always printing the literal value as a floating-point
    if literal_value.fract() == 0.0 {
        // If it's an integer, print as an integer i.e. 2.0
        println!("NUMBER {} {:.1}", number, literal_value);
    } else {
        // If it's an float, print exact percision
        println!("NUMBER {} {}", number, literal_value);
    }
}


fn handle_string_literal(chars: &mut std::iter::Peekable<std::str::Chars>, line: &mut i32) -> Result<(), ()> {
    let mut value = String::new();
    chars.next(); // Consume the opening quote

    while let Some(&char) = chars.peek() {
        if char == '"' {
            break;
        }
        if char == '\n' {
            *line += 1;
        }
        value.push(char);
        chars.next();
    }

    if chars.peek() != Some(&'"') {
        writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line).unwrap();
        return Err(());
    }

    chars.next(); // Consume the closing quote
    println!("STRING \"{}\" {}", value, value);
    Ok(())
}
