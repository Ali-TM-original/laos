use crate::lexer::lex::Token;
use std::char;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Intepreter {
    programcounter: usize,
    startline: usize,
    tokens: Vec<Token>,
    accumulator: i64,
    variables: HashMap<String, i64>,
    iscompareable: bool,
}

impl Intepreter {
    pub fn new(tokens: Vec<Token>, startline: usize, variables: HashMap<String, i64>) -> Self {
        Intepreter {
            programcounter: 0,
            tokens,
            startline,
            accumulator: 0,
            variables,
            iscompareable: false,
        }
    }
    pub fn interpret(&mut self) {
        self.eventloop();
    }
    pub fn eventloop(&mut self) {
        while self.tokens[self.programcounter] != Token::End
            && self.tokens[self.programcounter] != Token::Endprog
        {
            match self.tokens[self.programcounter] {
                Token::Ldm => println!("Found LDM"),
                Token::Ldd => self.interpret_ldd(),
                Token::Ldi => println!("Found LDI"),
                Token::Ldx => println!("Found LDX"),
                Token::Ldr => println!("Foud LDR"),
                Token::Mov => println!("Found MOV"),
                Token::Sto => println!("Found STO"),
                Token::Add => println!("Found ADD"),
                Token::Inc => self.interpret_inc(),
                Token::Cmp => self.interpret_cmp(),
                Token::Jpe => self.interpret_jpe(),
                Token::Jpn => println!("Found JPN"),
                Token::Jmp => println!("Found JMP"),
                Token::Out => self.interpret_out(),
                Token::End => println!("Found the Ending"),
                Token::Lsl => println!("Found LSL"),
                Token::Lsr => println!("Found LSR"),
                _ => panic!("Dead"),
            }
        }
    }
    fn interpret_inc(&mut self) {
        self.programcounter += 1;
        let mut num_val = true;
        if self.compare_identifiers(&Token::Identifier("".to_string())) == false {
            num_val = false
        }

        let idef = get_identifier(self.tokens.clone(), self.programcounter, num_val);
        let mut val = *self.variables.get(&idef).unwrap();
        val += 1;
        self.variables.insert(idef, val);
        self.programcounter += 1;
        println!("{:?}", self.variables);
    }
    fn interpret_cmp(&mut self) {
        let ans: bool = self.compare_acc();
        if ans == true {
            self.iscompareable = true
        }
    }

    fn interpret_jpe(&mut self) {
        // Complete this when you get time this is incomplete
        self.programcounter += 1;
        if self.iscompareable == true {
            let address: i64 = self.get_address_or_value();
            let addr = self.variables.get(&address.to_string()).unwrap().to_owned();
            println!("move to {} : startline {}", addr, self.startline);
            self.iscompareable = false
        } else {
            self.programcounter += 1;
            println!("{}", self.tokens[self.programcounter]);
        }
    }
    fn interpret_out(&mut self) {
        let char = decimals_to_string(self.accumulator as u8);
        println!("{}", char);
        self.programcounter += 1
    }
    fn interpret_ldd(&mut self) {
        let address: i64 = self.get_address_or_value();
        self.accumulator = self.variables.get(&address.to_string()).unwrap().to_owned();
    }

    fn compare_identifiers(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.tokens[self.programcounter]) == std::mem::discriminant(token)
    }
    fn compare_acc(&mut self) -> bool {
        let cmp: i64 = self.get_address_or_value();
        if self.accumulator == cmp {
            return true;
        } else {
            return false;
        }
    }
    fn get_address_or_value(&mut self) -> i64 {
        self.programcounter += 1;
        let check: bool = self.compare_identifiers(&Token::Number(0));
        if !check {
            panic!(
                "{}",
                format!(
                    "Expected a Memory Address found {}",
                    &self.tokens[self.programcounter]
                )
            )
        }
        if let Token::Number(address) = self.tokens[self.programcounter] {
            self.programcounter += 1;
            return address;
        } else {
            todo!()
        }
    }
}

/**
 * 
 * Helper Functions start from here try to put them in their own struct or just file is enough
 * 
 * 
*/
fn decimals_to_string(decimal: u8) -> String {
    println!("{}", decimal);
    let mut text = String::new();
    text.push(decimal as char);
    text
}

fn get_identifier(tokens: Vec<Token>, programcounter: usize, addr_as_string: bool) -> String {
    let mut identifier: String = String::new();
    if addr_as_string == true {
        if let Token::Identifier(string) = tokens[programcounter].clone() {
            identifier = string;
        }
        println!("Returning String");
        return identifier;
    } else {
        if let Token::Number(string) = tokens[programcounter].clone() {
            identifier = string.to_string();
        }
        println!("Returning Number {}", identifier);
        return identifier;
    }
}
