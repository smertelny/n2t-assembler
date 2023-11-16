use std::io;

use crate::{command::Command, symbol_table::SymbolTable};

#[derive(Debug)]
pub struct Parser<'a> {
    pub symbol_table: SymbolTable<'a>,
    pub commands: Vec<Command<'a>>,
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

                if let Some(command) = &instruction {
                    match command {
                        Command::A(value) if !value.chars().all(|c| c.is_numeric()) => {
                            let index = self.symbol_table.symbol_index;
                            self.symbol_table.table.entry(value).or_insert_with(|| {
                                self.symbol_table.symbol_index += 1;
                                index
                            });
                        }
                        _ => {}
                    }
                }

                instruction
            })
            .collect();
    }
}
