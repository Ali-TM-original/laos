use std::env;
use std::fs::read_to_string;
use crate::lexer::lex;
mod lexer;
mod tests;

/*
    1>. Control Tokens --> DOne
    2>. create tokens for assigning memory location a value
    3>. Create a hashmap of all memory variables
    4>. Parse Variables and store in the hashmap


*/

fn main() {
    // This is just for now at the end we use Clap to wrap up and create a CLI application
    let file: String = env::args().nth(1).unwrap();
    let content:String = read_to_string(file).unwrap();
    let tokens:Vec<lex::Token> = lex::generatetokens(content.as_str());
    println!("{:?}", tokens);

}
