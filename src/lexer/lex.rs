use std::panic;
use logos::{Logos};


/* 

    lets have a specific token at start and end of our code file like for example 


    START
        VARIABLES   
            variable 1
            variable 2
        ENDVARIABLES
        ALL OTHER ASM CODE
    END

    basic goal is to create an makeshift interpreter for the assembly code given to students
    in CAIE A-Level examinations. 
    This program specifically helps fill the table in assembly related questions

    At the end we need a way to draw the table out


*/

// It is always suppose to return a vector of tokens so no result expected
pub fn generatetokens(source_code:&str)-> Vec<Token>{
    let mut tok:Vec<Token> = Token::lexer(source_code).collect();
    let last_token = tok.last().unwrap().clone();
    println!("{:?}",tok);
    if tok[0] != Token::Startprog && last_token != Token::Endprog{
        panic!("Please ensure you code start with STARTPROG and ends with ENDPROG")
    }
    tok.remove(0); // remove Briyani kholo
    tok.pop();  // removes end token 
    tok
}


#[derive(Debug, Clone, Logos, PartialEq)]
#[allow(dead_code)]
pub enum Token{


    // Without these two tokens progam won't be interpreted
    #[token("STARTPROG")]
    Startprog,
    #[token("ENDPROG")]
    Endprog,

    

    // End of file, tokens to skip and errors riht here
    Eof,
    #[error]
    #[regex(r"[ \r\t\n\f]+", logos::skip)]
    #[regex(r"#\s?(.*)", logos::skip)]
    Error
}