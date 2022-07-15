mod lexer;
mod tests;
mod var;
mod interpret;

use crate::lexer::lex;
use crate::var::variableparser::Varparse;
use crate::interpret::interpreter::Intepreter;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let file: String = env::args().nth(1).unwrap();
    let content: String = read_to_string(file).unwrap();
    let tokens: Vec<lex::Token> = lex::generatetokens(content.as_str());
    let res: HashMap<String, i64> = HashMap::new();
    let mut variables = Varparse::new(tokens, res);
    let (var, tok) = variables.parse(); 
    let mut inter = Intepreter::new(tok, var.get("StartingPos").unwrap().clone() as usize, var);
    inter.interpret();
}
