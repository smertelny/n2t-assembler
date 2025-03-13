use std::io::{Read, Result, Write};

use crate::tokenizer::{Keyword, Token};

use super::CompilationEngine;

pub trait CompilationWriter {
    fn write_term(&mut self) -> Result<()>;
    fn write_expression_list(&mut self) -> Result<()>;
    fn write_expression(&mut self) -> Result<()>;
    fn write_closing_tag(&mut self, data: &[u8]) -> Result<()>;
    fn write_open_tag(&mut self, data: &[u8]) -> Result<()>;
    fn write_return_statement(&mut self) -> Result<()>;
    fn write_do_statement(&mut self) -> Result<()>;
    fn expect(&self, token: &Token, expected_token: Token);
    fn write_while_statement(&mut self) -> Result<()>;
    fn write_if_statement(&mut self) -> Result<()>;
    fn write_class(&mut self) -> Result<()>;
    fn write_terminal(&mut self, token: &Token) -> Result<()>;
    fn write_class_var_declarations(&mut self) -> Result<()>;
    fn write_subroutine_declarations(&mut self) -> Result<()>;
    fn write_parameter_list(&mut self) -> Result<()>;
    fn write_subroutine_body(&mut self) -> Result<()>;
    fn write_var_declaration(&mut self) -> Result<()>;
    fn write_statements(&mut self) -> Result<()>;
    fn write_let_statement(&mut self) -> Result<()>;
}

pub struct XmlWriter<'a, T: Read, W: Write> {
    compilation_engine: &'a mut CompilationEngine<T>,
    writer: &'a mut W,
    padding: u8,
    padding_level: u8,
}

impl<'a, T: Read, W: Write> XmlWriter<'a, T, W> {
    pub fn new(
        compilation_engine: &'a mut CompilationEngine<T>,
        writer: &'a mut W,
    ) -> XmlWriter<'a, T, W> {
        XmlWriter {
            compilation_engine,
            writer,
            padding: 0,
            padding_level: 2,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = self
            .compilation_engine
            .tokenizer
            .advance()
            .expect("no input found")
            .expect("failed to get first token");

        token
    }

    fn peek_token(&mut self) -> Token {
        self.compilation_engine
            .tokenizer
            .peek_token()
            .expect("no input found")
            .expect("failed to peek next token")
    }
}

impl<'a, T: Read, W: Write> CompilationWriter for XmlWriter<'a, T, W> {
    fn expect(&self, token: &Token, expected_token: Token) {
        if token != &expected_token {
            panic!(
                "unexpected token error: expected {:?}, found {:?}",
                expected_token, token
            );
        }
    }

    fn write_open_tag(&mut self, data: &[u8]) -> Result<()> {
        for _ in 0..self.padding {
            self.writer.write_all(b" ")?;
        }
        self.writer.write(data)?;
        self.writer.write_all(b"\n")?;

        self.padding = self.padding + self.padding_level;

        Ok(())
    }

    fn write_closing_tag(&mut self, data: &[u8]) -> Result<()> {
        self.padding = self.padding - self.padding_level;

        for _ in 0..self.padding {
            self.writer.write_all(b" ")?;
        }
        self.writer.write(data)?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }

    fn write_class(&mut self) -> Result<()> {
        let token = self // fn write_terminal(&self);
            .compilation_engine
            .tokenizer
            .advance()
            .expect("no input found")
            .expect("failed to get first token");

        let result = match token {
            Token::Keyword(Keyword::Class) => true,
            _ => false,
        };
        if !result {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "first token was not `class`",
            ));
        }

        self.write_open_tag(b"<class>")?;

        self.write_terminal(&token)?;

        let token = self.next_token();
        self.write_terminal(&token)?; // class name

        let token = self.next_token();
        self.write_terminal(&token)?; // `{` symbol

        let mut token = self.peek_token();
        loop {
            let has_var_declarations = match token {
                Token::Keyword(Keyword::Static) | Token::Keyword(Keyword::Field) => true,
                _ => false,
            };

            if has_var_declarations {
                self.write_class_var_declarations()?;
                token = self.peek_token();
            } else {
                break;
            }
        }

        loop {
            let has_subroutine_declarations = match token {
                Token::Keyword(Keyword::Constructor)
                | Token::Keyword(Keyword::Function)
                | Token::Keyword(Keyword::Method) => true,
                _ => false,
            };

            if has_subroutine_declarations {
                self.write_subroutine_declarations()?;
                token = self.peek_token();
            } else {
                break;
            }
        }

        let token = self.next_token();
        self.expect(&token, Token::Symbol(&'}'));
        self.write_terminal(&token)?;

        self.write_closing_tag(b"</class>")?;

        Ok(())
    }

    fn write_terminal(&mut self, token: &Token) -> Result<()> {
        for _ in 0..self.padding {
            self.writer.write_all(b" ")?;
        }

        match token {
            Token::Keyword(keyword) => {
                let tag = match keyword {
                    Keyword::Class => "class",
                    Keyword::Method => "method",
                    Keyword::Function => "function",
                    Keyword::Constructor => "constructor",
                    Keyword::Int => "int",
                    Keyword::Boolean => "boolean",
                    Keyword::Char => "char",
                    Keyword::Void => "void",
                    Keyword::Var => "var",
                    Keyword::Static => "static",
                    Keyword::Field => "field",
                    Keyword::Let => "let",
                    Keyword::Do => "do",
                    Keyword::If => "if",
                    Keyword::Else => "else",
                    Keyword::While => "while",
                    Keyword::Return => "return",
                    Keyword::True => "true",
                    Keyword::False => "false",
                    Keyword::Null => "null",
                    Keyword::This => "this",
                };
                self.writer.write_all(b"<keyword>")?;
                self.writer.write_all(tag.as_bytes())?;
                self.writer.write_all(b"</keyword>\n")?;
            }
            Token::Symbol(symbol) => {
                let mut tmp = [0u8; 4];
                let result = match symbol {
                    '<' => "&lt;",
                    '>' => "&gt;",
                    '&' => "&amp;",
                    '"' => "&quot",
                    _ => symbol.encode_utf8(&mut tmp),
                };
                self.writer.write_all(b"<symbol>")?;
                self.writer.write_all(result.as_bytes())?;
                self.writer.write_all(b"</symbol>\n")?;
            }
            Token::Identifier(ident) => {
                self.writer.write_all(b"<identifier>")?;
                self.writer.write_all(ident.as_bytes())?;
                self.writer.write_all(b"</identifier>\n")?;
            }
            Token::IntConst(int) => {
                let int = int.to_string(); // FIXME: need to avoid allocation
                self.writer.write_all(b"<integerConstant>")?;
                self.writer.write_all(int.as_bytes())?;
                self.writer.write_all(b"</integerConstant>\n")?;
            }
            Token::StringConst(string) => {
                self.writer.write_all(b"<stringConstant>")?;
                self.writer.write_all(string.as_bytes())?;
                self.writer.write_all(b"</stringConstant>\n")?;
            }
        }

        Ok(())
    }

    fn write_class_var_declarations(&mut self) -> Result<()> {
        self.write_open_tag(b"<classVarDec>")?;
        while let Some(Ok(token)) = self.compilation_engine.tokenizer.advance() {
            let is_end = match token {
                Token::Symbol(';') => true,
                _ => false,
            };

            self.write_terminal(&token)?;

            if is_end {
                break;
            }
        }

        self.write_closing_tag(b"</classVarDec>")?;
        Ok(())
    }

    fn write_subroutine_declarations(&mut self) -> Result<()> {
        self.write_open_tag(b"<subroutineDec>")?;

        for _ in 0..4 {
            let token = self.next_token();
            self.write_terminal(&token)?;
        }

        self.write_parameter_list()?;

        let token = &self.next_token();
        self.write_terminal(token)?; // close bracket

        self.write_subroutine_body()?;

        self.write_closing_tag(b"</subroutineDec>")?;

        Ok(())
    }

    fn write_parameter_list(&mut self) -> Result<()> {
        self.write_open_tag(b"<parameterList>")?;

        loop {
            let token = self.peek_token();
            if matches!(token, Token::Symbol(')')) {
                break;
            }

            self.next_token();
            self.write_terminal(&token)?;
        }

        self.write_closing_tag(b"</parameterList>")?;

        Ok(())
    }

    fn write_subroutine_body(&mut self) -> Result<()> {
        self.write_open_tag(b"<subroutineBody>")?;

        let mut token = self.next_token();
        self.write_terminal(&token)?; // {

        token = self.peek_token();
        loop {
            if matches!(token, Token::Keyword(Keyword::Var)) {
                let t = self.next_token();
                self.write_open_tag(b"<varDec>")?;
                self.write_terminal(&t)?;
                self.write_var_declaration()?;
                self.write_closing_tag(b"</varDec>")?;
                token = self.peek_token();
            } else {
                break;
            }
        }

        self.write_statements()?;

        token = self.next_token();
        self.expect(&token, Token::Symbol(&'}'));
        self.write_terminal(&token)?;

        self.write_closing_tag(b"</subroutineBody>")?;
        Ok(())
    }

    fn write_var_declaration(&mut self) -> Result<()> {
        loop {
            let token = self.next_token();
            self.write_terminal(&token)?;

            if matches!(token, Token::Symbol(';')) {
                break;
            }
        }
        Ok(())
    }

    fn write_statements(&mut self) -> Result<()> {
        self.write_open_tag(b"<statements>")?;

        let token = self.peek_token();

        if !matches!(
            token,
            Token::Keyword(Keyword::Let)
                | Token::Keyword(Keyword::If)
                | Token::Keyword(Keyword::While)
                | Token::Keyword(Keyword::Do)
                | Token::Keyword(Keyword::Return)
        ) {
            self.write_closing_tag(b"</statements>")?;

            return Ok(());
        }

        let mut token = self.peek_token();
        loop {
            match token {
                Token::Keyword(Keyword::Let) => self.write_let_statement()?,
                Token::Keyword(Keyword::If) => self.write_if_statement()?,
                Token::Keyword(Keyword::While) => self.write_while_statement()?,
                Token::Keyword(Keyword::Do) => self.write_do_statement()?,
                Token::Keyword(Keyword::Return) => self.write_return_statement()?,
                _ => break,
            };

            token = self.peek_token();
        }
        self.write_closing_tag(b"</statements>")?;

        Ok(())
    }

    fn write_let_statement(&mut self) -> Result<()> {
        self.write_open_tag(b"<letStatement>")?;

        let token = self.next_token();
        self.expect(&token, Token::Keyword(Keyword::Let));
        self.write_terminal(&token)?;

        let mut token = self.next_token();
        if !matches!(&token, Token::Identifier(_)) {
            panic!("Expected identifier, found {:?}", token);
        }
        self.write_terminal(&token)?;

        token = self.next_token();
        if matches!(&token, Token::Symbol(&'[')) {
            self.write_terminal(&token)?;
            self.write_expression()?;

            token = self.next_token();
            self.expect(&token, Token::Symbol(&']'));
            self.write_terminal(&token)?;

            token = self.next_token();
        }

        self.expect(&token, Token::Symbol(&'='));
        self.write_terminal(&token)?;

        self.write_expression()?;

        token = self.next_token();
        self.expect(&token, Token::Symbol(&';'));
        self.write_terminal(&token)?;

        self.write_closing_tag(b"</letStatement>")?;
        Ok(())
    }

    fn write_if_statement(&mut self) -> Result<()> {
        self.write_open_tag(b"<ifStatement>")?;

        let token = self.next_token();
        self.expect(&token, Token::Keyword(Keyword::If));
        self.write_terminal(&token)?;

        let token = self.next_token();
        self.expect(&token, Token::Symbol(&'('));
        self.write_terminal(&token)?;

        self.write_expression()?;

        let token = self.next_token();
        self.expect(&token, Token::Symbol(&')'));
        self.write_terminal(&token)?;

        let token = self.next_token();
        self.expect(&token, Token::Symbol(&'{'));
        self.write_terminal(&token)?;

        self.write_statements()?;

        let token = self.next_token();
        self.expect(&token, Token::Symbol(&'}'));
        self.write_terminal(&token)?;

        let token = self.peek_token();
        if token == Token::Keyword(Keyword::Else) {
            let token = self.next_token();
            self.write_terminal(&token)?;

            let token = self.next_token();
            self.expect(&token, Token::Symbol(&'{'));
            self.write_terminal(&token)?;

            self.write_statements()?;

            let token = self.next_token();
            self.expect(&token, Token::Symbol(&'}'));
            self.write_terminal(&token)?;
        }

        self.write_closing_tag(b"</ifStatement>")?;

        Ok(())
    }

    fn write_while_statement(&mut self) -> Result<()> {
        self.write_open_tag(b"<whileStatement>")?;

        let token = self.next_token();
        self.expect(&token, Token::Keyword(Keyword::While));
        self.write_terminal(&token)?;

        let mut token = self.next_token();
        self.expect(&token, Token::Symbol(&'('));
        self.write_terminal(&token)?;

        self.write_expression()?;

        token = self.next_token();
        self.expect(&token, Token::Symbol(&')'));
        self.write_terminal(&token)?;

        token = self.next_token();
        self.expect(&token, Token::Symbol(&'{'));
        self.write_terminal(&token)?;

        self.write_statements()?;

        token = self.next_token();
        self.expect(&token, Token::Symbol(&'}'));
        self.write_terminal(&token)?;

        self.write_closing_tag(b"</whileStatement>")?;
        Ok(())
    }

    fn write_do_statement(&mut self) -> Result<()> {
        self.write_open_tag(b"<doStatement>")?;

        let token = self.next_token();
        self.expect(&token, Token::Keyword(Keyword::Do));
        self.write_terminal(&token)?;

        loop {
            let token = self.next_token();
            self.write_terminal(&token)?;

            if matches!(token, Token::Symbol('(')) {
                self.write_expression_list()?;
            }

            if matches!(token, Token::Symbol(&';')) {
                break;
            }
        }
        self.write_closing_tag(b"</doStatement>")?;
        Ok(())
    }

    fn write_return_statement(&mut self) -> Result<()> {
        self.write_open_tag(b"<returnStatement>")?;

        let token = self.next_token();
        self.expect(&token, Token::Keyword(Keyword::Return));
        self.write_terminal(&token)?;

        self.write_expression()?;

        let token = self.next_token();
        self.expect(&token, Token::Symbol(&';'));
        self.write_terminal(&token)?;

        self.write_closing_tag(b"</returnStatement>")?;
        Ok(())
    }

    fn write_expression(&mut self) -> Result<()> {
        let token = self.peek_token();

        fn is_terminal(token: &Token) -> bool {
            match token {
                Token::IntConst(_) | Token::StringConst(_) => true,
                Token::Keyword(Keyword::True)
                | Token::Keyword(Keyword::False)
                | Token::Keyword(Keyword::Null)
                | Token::Keyword(Keyword::This) => true,
                Token::Identifier(_) => true,
                Token::Symbol(&'(') => true,
                Token::Symbol(&'~') | Token::Symbol(&'-') => true,
                Token::Symbol(&';') | Token::Symbol(&',') | Token::Symbol(&')') => false,
                _ => false,
            }
        }

        let is_term = is_terminal(&token);
        if !is_term {
            return Ok(());
        }

        self.write_open_tag(b"<expression>")?;

        self.write_term()?;

        let token = self.peek_token();
        match token {
            Token::Symbol(&'+')
            | Token::Symbol(&'-')
            | Token::Symbol(&'*')
            | Token::Symbol(&'/')
            | Token::Symbol(&'&')
            | Token::Symbol(&'|')
            | Token::Symbol(&'<')
            | Token::Symbol(&'>')
            | Token::Symbol(&'=') => {
                let token = self.next_token();
                self.write_terminal(&token)?;

                self.write_term()?;
            }
            _ => {}
        }

        self.write_closing_tag(b"</expression>")?;

        Ok(())
    }

    fn write_term(&mut self) -> Result<()> {
        self.write_open_tag(b"<term>")?;

        let token = self.next_token();
        self.write_terminal(&token)?;

        match token {
            Token::Symbol(&'~') | Token::Symbol(&'-') => {
                self.write_term()?;
            }
            Token::Identifier(_) => {
                let mut next = self.peek_token();

                if next == Token::Symbol(&'[') {
                    let token = self.next_token();
                    self.write_terminal(&token)?;

                    self.write_expression()?;

                    let token = self.next_token();
                    self.expect(&token, Token::Symbol(&']'));
                    self.write_terminal(&token)?;
                }

                // subroutine call
                if next == Token::Symbol(&'.') {
                    let token = self.next_token();
                    self.write_terminal(&token)?;

                    let token = self.next_token();
                    if !matches!(token, Token::Identifier(_)) {
                        panic!("Expected identifier, found {:?}", token);
                    }
                    self.write_terminal(&token)?;

                    next = self.peek_token();
                }

                // Need to be last
                if next == Token::Symbol(&'(') {
                    let token = self.next_token();
                    self.write_terminal(&token)?;

                    self.write_expression_list()?;

                    let token = self.next_token();
                    self.expect(&token, Token::Symbol(&')'));
                    self.write_terminal(&token)?;
                }
            }
            Token::Symbol(&'(') => {
                self.write_expression()?;

                let token = self.next_token();
                self.expect(&token, Token::Symbol(&')'));
                self.write_terminal(&token)?;
            }
            _ => {}
        }

        self.write_closing_tag(b"</term>")?;

        Ok(())
    }

    fn write_expression_list(&mut self) -> Result<()> {
        self.write_open_tag(b"<expressionList>")?;

        loop {
            let mut token = self.peek_token();
            if matches!(token, Token::Symbol(')')) {
                break;
            }

            self.write_expression()?;

            token = self.peek_token();
            if token == Token::Symbol(&',') {
                token = self.next_token();
                self.write_terminal(&token)?;
            }
        }

        self.write_closing_tag(b"</expressionList>")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect() {
        let b = "class Main {}".as_bytes();
        let mut wb = [0u8; 1024];
        let mut wb = wb.as_mut_slice();
        let mut c = CompilationEngine::new(b);
        let w = XmlWriter::new(&mut c, &mut wb);

        w.expect(&Token::Keyword(Keyword::If), Token::Keyword(Keyword::If));
        w.expect(&Token::IntConst(1), Token::IntConst(1));
        w.expect(&Token::Symbol(&'"'), Token::Symbol(&'"'));
    }

    #[test]
    #[should_panic]
    fn test_expect_panics() {
        let b = "class Main {}".as_bytes();
        let mut wb = [0u8; 1024];
        let mut wb = wb.as_mut_slice();
        let mut c = CompilationEngine::new(b);
        let w = XmlWriter::new(&mut c, &mut wb);

        w.expect(&Token::Keyword(Keyword::If), Token::Keyword(Keyword::Do));
    }
}
