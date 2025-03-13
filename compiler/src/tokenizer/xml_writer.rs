use std::io::{Result, Write};

use super::Token;

pub trait SimpleXmlWriter {
    fn write_xml<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write;
}

impl SimpleXmlWriter for Token {
    fn write_xml<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        match self {
            Token::Keyword(keyword) => {
                let kw = match keyword {
                    super::Keyword::Class => "class",
                    super::Keyword::Method => "method",
                    super::Keyword::Function => "function",
                    super::Keyword::Constructor => "constructor",
                    super::Keyword::Int => "int",
                    super::Keyword::Boolean => "boolean",
                    super::Keyword::Char => "char",
                    super::Keyword::Void => "void",
                    super::Keyword::Var => "var",
                    super::Keyword::Static => "static",
                    super::Keyword::Field => "field",
                    super::Keyword::Let => "let",
                    super::Keyword::Do => "do",
                    super::Keyword::If => "if",
                    super::Keyword::Else => "else",
                    super::Keyword::While => "while",
                    super::Keyword::Return => "return",
                    super::Keyword::True => "true",
                    super::Keyword::False => "false",
                    super::Keyword::Null => "null",
                    super::Keyword::This => "this",
                };
                writer.write_all(b"<keyword>")?;
                writer.write_all(kw.as_bytes())?;
                writer.write_all(b"</keyword>")?;
            }
            Token::Symbol(symbol) => {
                // Avoiding allocation for chars
                let mut tmp = [0u8; 4];
                let s = match symbol {
                    '<' => "&lt;",
                    '>' => "&gt;",
                    '"' => "&quot;",
                    '&' => "&amp;",
                    _ => symbol.encode_utf8(&mut tmp),
                };

                writer.write_all(b"<symbol>")?;
                writer.write_all(s.as_bytes())?;
                writer.write_all(b"</symbol>")?;
            }
            Token::Identifier(text) => {
                writer.write_all(b"<identifier>")?;
                writer.write_all(text.as_bytes())?;
                writer.write_all(b"</identifier>")?;
            }
            Token::IntConst(int) => {
                let int = int.to_string(); // FIXME: need to avoid allocation
                writer.write_all(b"<integerConstant>")?;
                writer.write_all(int.as_bytes())?;
                writer.write_all(b"</integerConstant>")?;
            }
            Token::StringConst(string) => {
                writer.write_all(b"<stringConstant>")?;
                writer.write_all(string.as_bytes())?;
                writer.write_all(b"</stringConstant>")?;
            }
        }
        Ok(())
    }
}
