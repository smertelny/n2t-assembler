use nand2tetris_assembler::symbol_table::{InstructionTable, SymbolTable};
use std::{
    env,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
};

#[derive(Debug)]
pub struct Parser<'a> {
    symbol_table: SymbolTable<'a>,
    commands: Vec<Command<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Result<Self, io::Error> {
        let symbol_table = SymbolTable::new();

        Ok(Parser {
            symbol_table,
            commands: Vec::new(),
        })
    }

    pub fn parse(&mut self, file: &'a str) {
        // First pass for getting all labels
        let mut position = 0;

        let _ = file.lines().for_each(|line| {
            let line = line.trim();
            if line.len() == 0 || line.starts_with("//") {
                return;
            }

            if line.starts_with("(") && line.ends_with(')') {
                let label = line
                    .strip_prefix('(')
                    .expect("Already checked")
                    .strip_suffix(')')
                    .expect("Already checked");
                self.symbol_table.table.insert(label, position);
                position -= 1;
            }

            position += 1;
        });

        // Second pass
        self.commands = file
            .lines()
            .filter_map(|line| {
                let instruction = Command::parse_instruction(&line);
                // self.position += 1;

                if let Some(command) = &instruction {
                    match command {
                        Command::A(value) if !value.chars().all(|c| c.is_numeric()) => {
                            let index = self.symbol_table.symbol_index;
                            self.symbol_table.table.entry(value).or_insert_with(|| {
                                self.symbol_table.symbol_index += 1;
                                index
                            });
                        }
                        // Command::L(label, value) => {
                        // self.symbol_table.table.insert(label, *value);
                        // self.position -= 1;
                        // }
                        _ => {}
                    }
                }

                instruction
            })
            .collect();

        // self.commands.iter().for_each(|c| {
        //     match &c {
        //         Command::A(value) => {
        //             if !value.chars().all(|c| c.is_numeric()) {
        //                 self.symbol_table.table.insert(&value, 0);
        //             }
        //         }
        //         Command::L(label, value) => {

        //         }
        //     }
        // });
    }
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    A(&'a str),
    C {
        dest: Option<&'a str>,
        comp: Option<&'a str>,
        jump: Option<&'a str>,
    },
    L(&'a str),
}

impl<'a> Command<'a> {
    pub fn parse_instruction(buf: &str) -> Option<Command> {
        let buf = buf.trim();

        match buf {
            buf if buf.starts_with("//") || buf.len() == 0 => None,
            buf if buf.starts_with('(') && buf.ends_with(')') => {
                let label = buf
                    .strip_prefix('(')
                    .expect("Already checked")
                    .strip_suffix(')')
                    .expect("Already checked");
                Some(Command::L(label))
            }
            buf if buf.starts_with("@") => Some(Command::A(
                buf.strip_prefix("@")
                    .expect("Can't fail because alread checked"),
            )),
            buf if buf.contains('=') && buf.contains(";") => {
                let (dest, rest) = buf.split_once("=").expect("Already checked");
                let (comp, jmp) = rest.split_once(";").expect("Already checked");

                Some(Command::C {
                    dest: Some(dest),
                    comp: Some(comp),
                    jump: Some(jmp),
                })
            }
            buf if buf.contains('=') => {
                let (dest, comp) = buf.split_once('=').expect("Already checked");
                Some(Command::C {
                    dest: Some(dest),
                    comp: Some(comp),
                    jump: None,
                })
            }
            buf if buf.contains(";") => {
                let (comp, dest) = buf.split_once(";").expect("Already checked");
                Some(Command::C {
                    dest: None,
                    comp: Some(comp),
                    jump: Some(dest),
                })
            }
            _ => panic!("Unknown command"),
        }
    }
}

pub struct Assembler;

impl Assembler {
    pub fn emit(parser: &Parser, file_name: &Path) -> Result<(), io::Error> {
        let (file_name, _) = file_name
            .as_os_str()
            .to_str()
            .expect("Ok")
            .split_once(".")
            .expect("Must be file with .asm");
        let mut file = BufWriter::new(File::create(format!("{file_name}.hack"))?);

        let instruction_table = InstructionTable::new();

        parser.commands.iter().for_each(|command| match command {
            Command::A(value) => {
                let number: usize;

                if value.chars().all(|c| c.is_numeric()) {
                    number = value.parse::<usize>().expect("Checked to be numeric");
                } else {
                    number = *parser
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

fn main() -> Result<(), io::Error> {
    let mut args = env::args();

    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "no filename passed!"));
    }

    if args.len() > 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "too many arguments"));
    }

    let file_path = args.nth(1).unwrap();
    let file_path: &Path = file_path.as_ref();

    let mut buf = String::with_capacity(1024 * 8);

    BufReader::new(File::open(file_path)?).read_to_string(&mut buf)?;

    let file = buf;

    let mut parser = Parser::new()?;

    parser.parse(&file);

    Assembler::emit(&parser, file_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn do_not_parse_comments() {
        let result = Command::parse_instruction("// Hello, world!");
        assert!(result.is_none());
    }

    #[test]
    fn ignore_whitespaces() {
        assert!(Command::parse_instruction("    \n").is_none());
    }

    #[test]
    fn parse_a_instruction() {
        let result = Command::parse_instruction("   @1234    ");
        assert!(result.is_some());
        assert_eq!(result, Some(Command::A("1234")));
    }
}
