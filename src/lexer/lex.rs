use logos::{Lexer, Logos};
use std::fmt;

pub fn generatetokens(source_code: &str) -> Vec<Token> {
    let mut tok: Vec<Token> = Token::lexer(source_code).collect();
    let last_token = tok.last().unwrap().clone();
    assert!(
        !(tok[0] != Token::Startprog && last_token != Token::Endprog),
        "Please ensure you code start with STARTPROG and ends with ENDPROG"
    );
    tok.remove(0);
    tok
}

#[derive(Debug, Clone, Logos, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    #[token("STARTPROG")]
    Startprog,
    #[token("ENDPROG")]
    Endprog,

    // Variable Tokens
    #[token("VARIABLES")]
    Startvariable,
    #[token("ENDVARIABLES")]
    Endvariables,
    #[regex(r"[a-zA-Z_?]+", to_string)]
    Identifier(String),
    #[token("VAR")]
    Var,
    #[token("POSITION")]
    Position,
    #[token("STARTLINE")]
    Startline,
    #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
    Number(i64),

    // Main Code Tokens
    #[token("LDM")]
    Ldm,
    #[token("LDD")]
    Ldd,
    #[token("LDI")]
    Ldi,
    #[token("LDX")]
    Ldx,
    #[token("LDR")]
    Ldr,
    #[token("MOV")]
    Mov,
    #[token("STO")]
    Sto,
    #[token("ADD")]
    Add,
    #[token("INC")]
    Inc,
    #[token("CMP")]
    Cmp,
    #[token("JPE")]
    Jpe,
    #[token("JPN")]
    Jpn,
    #[token("JMP")]
    Jmp,
    #[token("OUT")]
    Out,
    #[token("LSL")]
    Lsl,
    #[token("LSR")]
    Lsr,
    #[token("END")]
    End,

    // End of file, tokens to skip and errors riht here
    Eof,
    #[error]
    #[regex(r"[ \r\t\n\f]+", logos::skip)]
    #[regex(r"#\s?(.*)", logos::skip)]
    Error,
}

// Will be used primarily for better error messages sooner or later
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::{
            Endprog, Endvariables, Identifier, Number, Position, Startprog, Startvariable, Var,Inc
        };
        match self.clone() {
            Startvariable => write!(f, "Startvariable"),
            Endvariables => write!(f, "EndVariable"),
            Startprog => write!(f, "Startprog"),
            Endprog => write!(f, "Endprog"),
            Identifier(v) => v.fmt(f),
            Var => write!(f, "Var"),
            Position => write!(f, "Position"),
            Inc=> write!(f, "Inc"),
            Number(v) => v.fmt(f),
            _ => todo!(),
        }
    }
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_string())
}

fn to_float(lex: &mut Lexer<Token>) -> Option<i64> {
    Some(lex.slice().parse().ok()?)
}
