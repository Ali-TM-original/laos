mod lexer;
mod tests;
mod var;

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use crate::lexer::lex;
use crate::var::variableparser::Varparse;
/*
    1>. Control Tokens --> Done
    2>. create tokens for assigning memory location a value --> Done
    3>. Create a hashmap of all memory variables --> Done
    4>. Parse Variables and store in the hashmap --> Done


*/

fn main() {
    // This is just for now at the end we use Clap to wrap up and create a CLI application
    let file: String = env::args().nth(1).unwrap();
    let content:String = read_to_string(file).unwrap();
    let tokens:Vec<lex::Token> = lex::generatetokens(content.as_str());

    let res:HashMap<String, i64> = HashMap::new();
    // No need for default acc and ix values only complicates things
    let mut variables = Varparse::new(tokens, res);
    variables.parse();

}
