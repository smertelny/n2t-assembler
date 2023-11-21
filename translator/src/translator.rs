use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write},
    path::Path,
};

use crate::parser::Parser;

pub struct Translator<'a> {
    file_path: &'a Path,
    parser: Parser,
}

impl<'a> Translator<'a> {
    pub fn new(filepath: &'a Path) -> Result<Self, io::Error> {
        let file = BufReader::new(File::open(&filepath)?);

        let parser = Parser::new(file);
        // while let Some(_) = parser.advance(){
        //     dbg!(&parser);
        // };

        Ok(Self {
            file_path: filepath,
            parser,
        })
    }

    pub fn emit(&mut self) -> Result<(), io::Error> {
        let path = self.file_path.parent().expect("Must be ok");
        let name = self.file_path.file_stem().expect("Must be ok");
        let mut file = BufWriter::new(File::create(path.join(name).join(".asm"))?);

        while let Some(command) = self.parser.advance() {
            file.write_all(command.to_string().as_bytes())?;
        }

        Ok(())
    }
}
