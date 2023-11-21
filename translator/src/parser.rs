use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, self},
};

#[derive(Debug)]
pub enum Command {
    Arithmetic(ArtithmeticOperation),
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Label,
    GOTO,
    If,
    Function,
    Return,
    Call,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use super::parser::{ArtithmeticOperation::*, Command::*};

        match self {
            Push { segment, index } => {
                // write!(f, "\n@{index}\nD=A\n@SP\nM=M+1\n\n{}", segment)
                match segment {
                    Segment::Constant => {
                        write!(f, "\n@{index}\nD=A")?;
                    }
                    _ => {},
                }

                write!(f, "\n@SP\nM=A\n@SP\nM=M+1")
            }
            _ => Err(fmt::Error),
        }
    }
}

pub struct NotCommandError;

impl fmt::Display for NotCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "supplied string is not a command")
    }
}

impl TryFrom<&str> for Command {
    type Error = NotCommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            op if ArtithmeticOperation::try_from(op).is_ok() => {
                Ok(Command::Arithmetic(ArtithmeticOperation::try_from(op)?))
            }
            op if op.starts_with("push") || op.starts_with("pop") => {
                let (command, rest) = op.split_once(" ").expect("Must be ok");
                let (segment, index) = rest.split_once(" ").expect("Must be ok too");

                let segment = Segment::try_from(segment)?;
                let index = index.parse::<u16>().map_err(|_| NotCommandError)?;

                if command == "push" {
                    return Ok(Command::Push { segment, index });
                } else if command == "pop" {
                    return Ok(Command::Pop { segment, index });
                } else {
                    unreachable!()
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug)]
enum ArtithmeticOperation {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl TryFrom<&str> for ArtithmeticOperation {
    type Error = NotCommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" => Ok(ArtithmeticOperation::Add),
            "sub" => Ok(ArtithmeticOperation::Sub),
            "neg" => Ok(ArtithmeticOperation::Neg),
            "eq" => Ok(ArtithmeticOperation::Eq),
            "gt" => Ok(ArtithmeticOperation::Gt),
            "lt" => Ok(ArtithmeticOperation::Lt),
            "and" => Ok(ArtithmeticOperation::And),
            "or" => Ok(ArtithmeticOperation::Or),
            "not" => Ok(ArtithmeticOperation::Not),
            _ => Err(NotCommandError),
        }
    }
}

#[derive(Debug)]
enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl TryFrom<&str> for Segment {
    type Error = NotCommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use self::Segment::*;

        match value {
            "argument" => Ok(Argument),
            "local" => Ok(Local),
            "static" => Ok(Static),
            "constant" => Ok(Constant),
            "this" => Ok(This),
            "that" => Ok(That),
            "pointer" => Ok(Pointer),
            "temp" => Ok(Temp),
            _ => Err(NotCommandError),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Segment::*;

        match self {
            Constant => write!(f, ""),
            _ => Err(fmt::Error),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Parser {
    lines: Lines<BufReader<File>>,
    // command: Command,
}

impl Parser {
    pub(crate) fn new(file: BufReader<File>) -> Self {
        Self {
            // command: Command::Arithmetic(ArtithmeticOperation::Add),
            lines: file.lines(),
        }
    }

    pub(crate) fn advance(&mut self) -> Option<Command> {
        // let mut buf: [u8; 1024] = [];
        let mut line = self.lines.next()?.expect("Must be ok");
        while line.trim().starts_with("//") || line.trim().is_empty() {
            line = self.lines.next()?.expect("Must be ok");
        }

        let line = line.trim();

        Command::try_from(line).ok()
    }
}
