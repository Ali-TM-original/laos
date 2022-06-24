use std::collections::HashMap;

use crate::lexer::lex::Token;


/*

    For Now this does nothing this only marks the template used
    currently figuring out best approach to solve our little table
    problem

*/

#[allow(dead_code)]
pub struct Intepreter{
    programcounter: i64, // used to move between lines
    tokens:Vec<Token>, // contains all tokens as usual
    accumulator:i64, // holds working
    variables:HashMap<String, i64>
    // Have another field called terminal later to print out the table
}

impl Intepreter{
    pub fn new(tokens: Vec<Token>, programcounter:i64, variables:HashMap<String, i64>)->Self{
        Intepreter { programcounter , tokens, accumulator: 0, variables }
    }
}