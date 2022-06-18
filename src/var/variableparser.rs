/*

    this module will be used to parse out the variables in our file

    Since we don't have to keep track of the counter in the Vector since we will be 
    removing all the tokens till the Endvariables token

    it works by checking if the Endvariable token has been reached in our vector
    assuming Endvariable and startvariable tokens exist

    it then removes the first token then reads identifier removes a token again reads value 
    and removes a token again

    that way all tokens corresponding to a single declaration are removed for example 

    VAR IX 20  all three tokens are removed


*/

use std::collections::HashMap;
use std;
use crate::lexer::lex::Token;

pub type Identifier = String;

#[allow(dead_code)]
pub struct Varparse{
    tokens:Vec<Token>,
    variables:HashMap<String, i64>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Varparse{
    pub fn new(tokens:Vec<Token>, variables:HashMap<String, i64>)->Varparse{
        let varparse = Varparse { tokens, variables};
        varparse
    }
    pub fn parse(& mut self){
        // for now we are not doing proper error handeling 
        // lets make this beautiful at the end
        let _ = self.pass_or_panic(Token::Startvariable, self.tokens[0].clone());
        self.tokens.remove(0);
    
        // Now lets start looping right here

        self.eventloop();
        println!("{:?}", self.variables);
    }
    fn pass_or_panic(&self, expected:Token, got:Token)->Token{
        if expected != got{
            panic!("{}",format!("Expected Token:{} Found Token:{}", expected, got));
        }
        return got
    }
    fn eventloop(& mut self){
        // tbh this is stupid fix this later 
        while self.tokens[0] != Token::Endvariables{
            if self.tokens[0] == Token::Var{
                self.getvariables();
            }
        }
    }
    fn getvariables(&mut self){
        self.tokens.remove(0); // this gets us the name
        let tok = self.compare_identifiers(Token::Identifier("".to_string()));
        if !tok{
            panic!("{}",format!("Expected Token:Identifier Found TOken:{}", self.tokens[0].clone()).as_str())
        }
        let iden:Identifier = self.get_identifier();
        self.tokens.remove(0); // now we get value
        let an = self.compare_identifiers(Token::Number(0));
        if !an{
            panic!("{}",format!("Expected Token:Number Found TOken:{}", self.tokens[0].clone()).as_str())
        }

        let num:i64 = self.get_value();
        self.variables.insert(iden, num);
        self.tokens.remove(0);
    }
    fn compare_identifiers(&self, token: Token) -> bool {
        //println!("Compare This {} To This {}", self.tokens[self.pc], token);
        std::mem::discriminant(&self.tokens[0]) == std::mem::discriminant(&token)
    }
    fn get_identifier(&mut self)->Identifier{
        let mut identifier:Identifier = String::new();
        if let Token::Identifier(string) = self.tokens[0].clone() {
            identifier = string;
        }
        return identifier
    }
    fn get_value(&mut self)->i64{
        let mut num:i64 = 0;
        if let Token::Number(string) = self.tokens[0].clone() {
            num = string;
        }
        num
    }
}