use std::io::Write;
use crate::lexer::lexer::Lexer;

mod lexer;

const PROMPT: &str = ">> ";

fn main() {
    let user = whoami::username();

    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");
    start();
}

fn start() {
    let mut input = String::new();
    loop {
        print!("{}", PROMPT);
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).unwrap();
        let lexer = Lexer::new(&mut input);

        for token in lexer {
            println!("{:?}: {}", token, token);
        }
    }
}