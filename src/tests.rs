#[allow(unused_imports)]
use crate::lexer::lex;
#[allow(unused_imports)]
use logos::Logos;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn can_it_tokenize() {
        let mut lexer = lex::Token::lexer("STARTPROG ENDPROG");
        assert_eq!(lexer.next(), Some(lex::Token::Startprog));
        assert_eq!(lexer.next(), Some(lex::Token::Endprog));
    }
    #[test]
    fn can_it_comment() {
        let mut lexer = lex::Token::lexer("STARTPROG #hello world ENDPROG");
        assert_eq!(lexer.next(), Some(lex::Token::Startprog))
    }
}
