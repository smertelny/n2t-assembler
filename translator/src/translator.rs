use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::{Path, PathBuf},
};

use crate::parser::Parser;

pub struct Translator {
    files: Vec<PathBuf>, // Must not contain more than 256 vm files
                         // parser: Parser,
}

impl Translator {
    pub fn new(filepath: &Path) -> Result<Self, io::Error> {
        let mut files = vec![];

        if filepath.is_dir() {
            filepath.read_dir().expect("Must be ok").for_each(|item| {
                if let Ok(item) = item {
                    if let Some(ext) = item.path().extension() {
                        if ext == "vm"
                            && !item
                                .path()
                                .file_name()
                                .expect("Must have filename")
                                .to_str()
                                .expect("Must convert to str normally")
                                .starts_with(".")
                        {
                            files.push(item.path())
                        }
                    }
                }
            });

            if files.len() == 0 {
                panic!(
                    "No .vm files found in provided path: {}",
                    filepath.as_os_str().to_string_lossy()
                );
            }
        } else {
            files.push(filepath.to_path_buf());
        }

        Ok(Self { files })
    }

    pub fn emit<T: Write>(&mut self, mut writer: T) -> Result<(), io::Error> {
        let mut parser = Parser::new(
            BufReader::new(File::open(
                self.files.iter().next().expect("It can't be empty"),
            )?),
            "".to_owned(),
        );
        // Initializing stack. Must be included once
        writeln!(writer, "@256")?;
        writeln!(writer, "D=A")?;
        writeln!(writer, "@SP")?;
        writeln!(writer, "M=D")?;

        // Here we need to CALL Sys.init, not just jump to it
        // writeln!(writer, "@Sys.init")?;
        // writeln!(writer, "0;JMP")?;
        writer.write_all(crate::parser::Command::init().to_string().as_bytes())?;

        self.files.iter().try_for_each(|filepath| {
            let file = BufReader::new(File::open(&filepath)?);
            let name = filepath.file_name().unwrap().to_string_lossy().to_string();
            parser.next_file(file, &name);

            writeln!(writer, "// File: {name}")?;

            while let Some(command) = parser.advance() {
                writer.write_all(command.to_string().as_bytes())?;
            }

            Ok::<(), io::Error>(())
        })?;

        Ok(())
    }
}
