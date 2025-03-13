mod writer;

use writer::{CompilationWriter, XmlWriter};

use crate::tokenizer::Tokenizer;

pub enum NonTerminals {
    Class,
    ClassVarDec,
    SubroutineDec,
    ParameterList,
    SubroutineBody,
    VarDec,
    Statements,
    WhileStatement,
    IfStatement,
    ReturnStatement,
    LetStatement,
    DoStatement,
    Expression,
    Term,
    ExpressionList,
}

// struct Class {
//     name: String,
//     vars: Vec<()>,
//     subroutines: Vec<()>,
// }

/// Recursive top-down parser
pub struct CompilationEngine<T: std::io::Read> {
    tokenizer: Tokenizer<T>,
}

impl<T: std::io::Read> CompilationEngine<T> {
    pub fn new(file: T) -> CompilationEngine<T> {
        CompilationEngine {
            tokenizer: Tokenizer::new(file).expect("failed to create tokenizer"),
        }
    }

    pub fn compile<W: std::io::Write>(&mut self, mut writer: W) -> std::io::Result<()> {
        let mut output_writer = XmlWriter::new(self, &mut writer);
        output_writer.write_class()?;

        // let name = self
        //     .tokenizer
        //     .advance()
        //     .expect("input ended unexpectedly, class name expected")?;
        // let name = match name {
        //     Token::StringConst(name) => name,
        //     _ => panic!("expected class name, found {:?}", name),
        // };

        Ok(())
    }
}
