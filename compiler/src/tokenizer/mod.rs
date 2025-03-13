pub mod xml_writer;

use core::panic;
use std::io::{BufRead, BufReader, Result};

static SYMBOL_LIST: [char; 19] = [
    '(', ')', '{', '}', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];

/// Using this instead of `str.trim()` because
/// it trims string without realocation in place
#[inline]
fn trim(s: &mut String) {
    let trimmed = s.trim_end();
    s.truncate(trimmed.len());

    let trimmed = s.trim_start();
    s.replace_range(..(s.len() - trimmed.len()), "");
}

struct CharsNumber(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(&'static char),
    Identifier(String),
    IntConst(u16),
    StringConst(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

#[derive(Debug)]
pub struct Tokenizer<T> {
    file: BufReader<T>,
    buf: String,
}

impl<T: std::io::Read> Tokenizer<T> {
    pub fn new(file: T) -> std::io::Result<Tokenizer<T>> {
        Ok(Tokenizer {
            file: BufReader::new(file),
            buf: String::with_capacity(2048),
        })
    }

    fn get_next_token(&mut self, delete: bool) -> Option<Result<Token>> {
        // `while let` do not make possibility to get error from read_line
        loop {
            if self.buf.len() == 0 {
                let bytes = self.file.read_line(&mut self.buf);
                let bytes = match bytes {
                    Ok(n) => n,
                    Err(e) => return Some(Err(e)),
                };
                if bytes == 0 {
                    return None;
                }
            }

            // Clearing comments
            trim(&mut self.buf);

            let mut line = self.buf.as_str();

            if line.contains("//") {
                let index = line.find("//").expect("checked previously");
                self.buf.replace_range(index.., "");
                trim(&mut self.buf);
                line = self.buf.as_str();
            }

            if line.contains("/*") {
                if line.contains("*/") {
                    let start = line.find("/*").expect("checked");
                    let end = line.find("*/").expect("checked");

                    self.buf.replace_range(start..(end + "*/".len()), "");
                    line = self.buf.trim();
                } else {
                    while let Ok(bytes) = self.file.read_line(&mut self.buf) {
                        if bytes == 0 {
                            return None;
                        }

                        if self.buf.contains("*/") {
                            break;
                        }
                    }

                    let start = self.buf.find("/*").expect("checked");
                    let end = self.buf.find("*/").expect("checked");

                    self.buf.replace_range(start..(end + "*/".len()), "");
                    line = self.buf.trim();
                }
            }
            if line.len() == 0 {
                continue;
            }

            let (n, token) = Self::parse_token(line);
            if delete {
                self.buf.replace_range(..n.0, "");
            }

            return Some(Ok(token));
        }
    }

    pub fn advance(&mut self) -> Option<Result<Token>> {
        const DELETE: bool = true;
        self.get_next_token(DELETE)
    }

    pub(crate) fn peek_token(&mut self) -> Option<Result<Token>> {
        const DELETE: bool = false;
        self.get_next_token(DELETE)
    }

    fn parse_token(line: &str) -> (CharsNumber, Token) {
        let first_symbol = line.chars().nth(0).expect("already checked");
        if first_symbol.is_alphabetic() {
            Self::parse_keyword_or_identifier(&line)
        } else if first_symbol.is_ascii_digit() {
            Self::parse_nubmer(&line)
        } else if let Some(symbol) = SYMBOL_LIST.iter().find(|c| **c == first_symbol) {
            (CharsNumber(1), Token::Symbol(symbol))
        } else if first_symbol == '"' {
            let line = &line[1..];
            let end_string_index = line.find('"').expect("could not find closing \"");
            let string = String::from(&line[..end_string_index]);
            // +2 needed because of starting end closing " symbols
            (CharsNumber(string.len() + 2), Token::StringConst(string))
        } else {
            panic!(
                "unknown symbol occured \"{}\" in line {}",
                first_symbol, line
            );
        }
    }

    fn parse_keyword_or_identifier(line: &str) -> (CharsNumber, Token) {
        let token;
        if let Some(index) = line.find(|c: char| !(c.is_alphabetic() || c == '_')) {
            token = &line[..index]; // TODO: may be bug here
        } else {
            token = &line;
        }

        use Keyword::*;
        match token {
            "class" => (CharsNumber("class".len()), Token::Keyword(Class)),
            "method" => (CharsNumber("method".len()), Token::Keyword(Method)),
            "function" => (CharsNumber("function".len()), Token::Keyword(Function)),
            "constructor" => (
                CharsNumber("constructor".len()),
                Token::Keyword(Constructor),
            ),
            "int" => (CharsNumber("int".len()), Token::Keyword(Int)),
            "boolean" => (CharsNumber("boolean".len()), Token::Keyword(Boolean)),
            "char" => (CharsNumber("char".len()), Token::Keyword(Char)),
            "void" => (CharsNumber("void".len()), Token::Keyword(Void)),
            "var" => (CharsNumber("var".len()), Token::Keyword(Var)),
            "static" => (CharsNumber("static".len()), Token::Keyword(Static)),
            "field" => (CharsNumber("field".len()), Token::Keyword(Field)),
            "let" => (CharsNumber("let".len()), Token::Keyword(Let)),
            "do" => (CharsNumber("do".len()), Token::Keyword(Do)),
            "if" => (CharsNumber("if".len()), Token::Keyword(If)),
            "else" => (CharsNumber("else".len()), Token::Keyword(Else)),
            "while" => (CharsNumber("while".len()), Token::Keyword(While)),
            "return" => (CharsNumber("return".len()), Token::Keyword(Return)),
            "true" => (CharsNumber("true".len()), Token::Keyword(True)),
            "false" => (CharsNumber("false".len()), Token::Keyword(False)),
            "null" => (CharsNumber("null".len()), Token::Keyword(Null)),
            "this" => (CharsNumber("this".len()), Token::Keyword(This)),
            _ => (
                CharsNumber(token.len()),
                Token::Identifier(String::from(token)),
            ),
        }
    }

    fn parse_nubmer(line: &str) -> (CharsNumber, Token) {
        let number;
        if let Some(index) = line.find(|c: char| !c.is_ascii_digit()) {
            number = &line[..index];
        } else {
            number = &line;
        }

        match number.parse::<u16>() {
            Ok(result_number) => (CharsNumber(number.len()), Token::IntConst(result_number)),
            Err(err) => {
                panic!("Can not parse integer: {}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_basic_comments() {
        let buf = "// comment".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();
        // assert_eq!(t.advance(), None);
        assert!(t.advance().is_none());
    }

    #[test]
    fn trim_closing_comments() {
        let buf = "/* comment */".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(t.advance().is_none());
    }

    #[test]
    fn trim_closing_multiline_comments() {
        let buf = "/* comment \n*\n */if".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(matches!(t.advance(), Some(Ok(Token::Keyword(Keyword::If)))));
    }

    #[test]
    fn trim_multiline_api_comments() {
        let buf = "/** API comment \n* line\n *    \n*/12".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(matches!(t.advance(), Some(Ok(Token::IntConst(12)))));
    }

    #[test]
    fn parse_symbol() {
        let buf = "   , ".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(matches!(t.advance(), Some(Ok(Token::Symbol(',')))));
    }

    #[test]
    fn parse_multiple_tokens() {
        let buf = "  [12]".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(matches!(t.advance(), Some(Ok(Token::Symbol('[')))));
        assert!(matches!(t.advance(), Some(Ok(Token::IntConst(12)))));
        assert!(matches!(t.advance(), Some(Ok(Token::Symbol(']')))));
    }

    #[test]
    fn parse_multiple_multiline_tokens() {
        let buf = "  [12]\nclass".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(matches!(t.advance(), Some(Ok(Token::Symbol('[')))));
        assert!(matches!(t.advance(), Some(Ok(Token::IntConst(12)))));
        assert!(matches!(t.advance(), Some(Ok(Token::Symbol(']')))));
        assert!(matches!(
            t.advance(),
            Some(Ok(Token::Keyword(Keyword::Class)))
        ));
    }

    #[test]
    fn parse_identifier() {
        let buf = "Main".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        let r = String::from("Main");
        let result = match t.advance() {
            Some(Ok(Token::Identifier(text))) => text == r,
            _ => false,
        };
        assert!(result);
    }

    #[test]
    fn parse_identifier_with_underscore() {
        let buf = "class Main_class".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(matches!(
            t.advance(),
            Some(Ok(Token::Keyword(Keyword::Class)))
        ));
        let r = String::from("Main_class");
        let result = match t.advance() {
            Some(Ok(Token::Identifier(text))) => text == r,
            _ => false,
        };
        assert!(result);
    }

    #[test]
    fn peek_token_does_not_consume_input() {
        let buf = "class Main_class".as_bytes();
        let mut t = Tokenizer::new(buf).unwrap();

        assert!(matches!(
            t.peek_token(),
            Some(Ok(Token::Keyword(Keyword::Class)))
        ));
        assert!(matches!(
            t.peek_token(),
            Some(Ok(Token::Keyword(Keyword::Class)))
        ));
    }
}
