use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use crate::{command::Command, parser::Parser, symbol_table::InstructionTable};

pub struct Assembler<'a> {
    parser: Parser<'a>,
}

impl<'a> Assembler<'a> {
    pub fn new(file: &'a str) -> Self {
        let mut parser = Parser::<'a>::new().unwrap();
        parser.parse(file);
        Self { parser }
    }

    pub fn emit(&self, file_name: &Path) -> Result<(), io::Error> {
        let (file_name, _) = file_name
            .as_os_str()
            .to_str()
            .expect("Ok")
            .split_once(".")
            .expect("Must be file with .asm");
        let mut file = BufWriter::new(File::create(format!("{file_name}.hack"))?);

        let instruction_table = InstructionTable::new();

        self.parser
            .commands
            .iter()
            .for_each(|command| match command {
                Command::A(value) => {
                    let number: usize;

                    if value.chars().all(|c| c.is_numeric()) {
                        number = value.parse::<usize>().expect("Checked to be numeric");
                    } else {
                        number = *self
                            .parser
                            .symbol_table
                            .table
                            .get(value)
                            .expect("Value must be in symbol table")
                    }

                    let binary = format!("0{:015b}\n", number);
                    file.write_all(&binary.as_bytes())
                        .expect("Failed to write into file");
                }
                Command::C { dest, comp, jump } => {
                    let mut result: u16 = 0 | 0b111_00000_0000_0000;
                    if let Some(dest) = dest {
                        result |= instruction_table.dest.get(dest).expect("Unknown dest");
                    } else {
                        result |= instruction_table
                            .dest
                            .get("null")
                            .expect("Must be in HashMap");
                    }

                    if let Some(comp) = comp {
                        result |= instruction_table.comp.get(comp).expect("Unknown comp");
                    } else {
                        result |= instruction_table
                            .comp
                            .get("null")
                            .expect("Must be in HashMap");
                    }

                    if let Some(jump) = jump {
                        result |= instruction_table.jump.get(jump).expect("Unknown jump");
                    } else {
                        result |= instruction_table
                            .jump
                            .get("null")
                            .expect("Must be in HashMap");
                    }

                    file.write_all(format!("{:016b}\n", result).as_bytes())
                        .expect("Error while writing to file");
                }
                _ => {}
            });

        file.flush()?;

        Ok(())
    }
}
