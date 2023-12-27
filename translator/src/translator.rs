use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::Path,
};

use crate::parser::Parser;

pub struct Translator {
    parser: Parser,
}

impl Translator {
    pub fn new(filepath: &Path) -> Result<Self, io::Error> {
        let name = filepath.file_name().unwrap().to_string_lossy().to_string();
        let file = BufReader::new(File::open(&filepath)?);

        let parser = Parser::new(file, name);

        Ok(Self { parser })
    }

    pub fn emit<T: Write>(&mut self, mut writer: T) -> Result<(), io::Error> {
        while let Some(command) = self.parser.advance() {
            writer.write_all(command.to_string().as_bytes())?;
        }

        Ok(())
    }
}
