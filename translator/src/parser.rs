use core::fmt;
use std::{
    cell::Cell,
    io::{BufRead, Lines},
};

#[derive(Debug)]
pub struct Command {
    cmd: _Command,
    label_value: std::cell::Cell<u16>, // Need this because we can't change fmt::Display API
    call_label: std::cell::Cell<u16>,
    name: String,
}

impl Command {
    pub fn new(name: String) -> Self {
        Self {
            cmd: _Command::Return,
            label_value: std::cell::Cell::new(0),
            call_label: std::cell::Cell::new(0),
            name,
        }
    }
}

#[derive(Debug)]
enum _Command {
    Arithmetic(ArtithmeticOperation),
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Label(String),
    GOTO(String),
    If(String),
    Function { name: String, args: u16 },
    Return,
    Call { name: String, args: u16 },
}

impl _Command {
    /// Pops one value into D register and other will be in M register
    /// without decreasing SP
    #[inline]
    fn pop_before_op(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "// Pop two values from stack")?;
        writeln!(f, "@SP")?;
        writeln!(f, "A=M")?;
        writeln!(f, "A=A-1")?;
        writeln!(f, "D=M")?;
        writeln!(f, "A=A-1")?;

        Ok(())
    }

    #[inline]
    fn push_from_d_reg(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "// Push to stack from D reg")?;
        writeln!(f, "@SP")?;
        writeln!(f, "A=M")?; // Goto stack top
        writeln!(f, "M=D")?; // push data to stack
        writeln!(f, "D=A+1")?; // increment stack pointer
        writeln!(f, "@SP")?;
        writeln!(f, "M=D")?; // write stack top to stack pointer registry

        Ok(())
    }

    #[inline]
    fn pop_into_d_reg(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "@SP")?;
        writeln!(f, "AM=M-1")?;
        writeln!(f, "D=M")?;

        Ok(())
    }

    #[inline]
    fn compare_and_write(
        cmd: &ArtithmeticOperation,
        label_value: &Cell<u16>,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        use ArtithmeticOperation::*;
        let v = label_value.get();
        let label = format!("L{}", v);
        let ne_label = format!("L{}", v + 1);
        label_value.set(v + 2);

        writeln!(f, "D=M-D")?;
        writeln!(f, "@{}", label)?;
        write!(f, "D;")?;

        match cmd {
            Eq => writeln!(f, "JEQ")?,
            Lt => writeln!(f, "JLT")?,
            Gt => writeln!(f, "JGT")?,
            _ => panic!("Can't use jump with operation: {:?}", cmd),
        }

        writeln!(f, "D=0")?;
        writeln!(f, "@{}", ne_label)?;
        writeln!(f, "0;JMP")?;
        writeln!(f, "({label})")?;
        writeln!(f, "D=-1")?;
        writeln!(f, "({ne_label})")?;
        writeln!(f, "@SP")?;
        writeln!(f, "AM=M-1")?;
        writeln!(f, "A=A-1")?;
        writeln!(f, "M=D")?;

        Ok(())
    }

    #[inline]
    fn restore_sp(f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "D=A+1")?; // Inc stack pointer and store stack top in D reg
        writeln!(f, "@SP")?;
        writeln!(f, "M=D")?; // Write stack top to SP

        Ok(())
    }

    #[inline]
    fn register_to_stack(f: &mut fmt::Formatter, name: &str) -> fmt::Result {
        writeln!(f, "@{name}")?;
        writeln!(f, "D=M")?;
        writeln!(f, "@SP")?;
        writeln!(f, "AM=M+1")?;
        writeln!(f, "M=D")?;

        Ok(())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use super::parser::{ArtithmeticOperation::*, Segment::*, _Command::*};

        match &self.cmd {
            Push { segment, index } => {
                // write!(f, "\n@{index}\nD=A\n@SP\nM=M+1\n\n{}", segment)
                match segment {
                    Constant => {
                        writeln!(f, "// PUSH constant {index}")?;
                        writeln!(f, "@{index}")?;
                        writeln!(f, "D=A")?;
                    }
                    Static => {
                        writeln!(f, "@{}.{}", self.name, index)?;
                        writeln!(f, "D=M")?;
                    }
                    _ => {
                        match segment {
                            Constant | Static => unreachable!(),
                            Temp | Pointer => {
                                if let Pointer = segment {
                                    writeln!(f, "@THIS")?;
                                } else {
                                    writeln!(f, "@R5")?;
                                }

                                writeln!(f, "D=A")?;
                            }
                            Local | Argument | That | This => {
                                match segment {
                                    Local => writeln!(f, "@LCL")?,
                                    Argument => writeln!(f, "@ARG")?,
                                    This => writeln!(f, "@THIS")?,
                                    That => writeln!(f, "@THAT")?,
                                    _ => unreachable!(),
                                }

                                writeln!(f, "D=M")?;
                            }
                        }

                        writeln!(f, "@{index}")?;
                        writeln!(f, "A=D+A")?;
                        writeln!(f, "D=M")?;
                    }
                }

                // Push to stack from D register
                _Command::push_from_d_reg(f)?;
                Ok(())
            }
            Pop { segment, index } => {
                // writeln!(f)
                // writeln!(f, "D=M")?;
                match segment {
                    Constant => unreachable!(),
                    Static => {
                        writeln!(f, "@{}.{}", self.name, index)?;
                        writeln!(f, "D=A")?;
                    }
                    Temp | Pointer => {
                        match segment {
                            Temp => writeln!(f, "@R5")?,
                            Pointer => writeln!(f, "@THIS")?,
                            _ => unreachable!(),
                        }

                        writeln!(f, "D=A")?;
                        writeln!(f, "@{index}")?;
                        writeln!(f, "D=D+A")?;
                    }
                    _ => {
                        match segment {
                            Argument => writeln!(f, "@ARG")?,
                            Local => writeln!(f, "@LCL")?,
                            This => writeln!(f, "@THIS")?,
                            That => writeln!(f, "@THAT")?,
                            _ => unreachable!(),
                        };

                        // Calculate result RAM
                        writeln!(f, "D=M")?;
                        writeln!(f, "@{index}")?;
                        writeln!(f, "D=D+A")?;
                    }
                }

                // Save to R15 reg as temporary storage
                writeln!(f, "@R15")?;
                writeln!(f, "M=D")?;

                writeln!(f, "@SP")?;
                writeln!(f, "AM=M-1")?;
                writeln!(f, "D=M")?;

                writeln!(f, "@R15")?;
                writeln!(f, "A=M")?;
                writeln!(f, "M=D")?;

                Ok(())
            }
            Arithmetic(op) => {
                _Command::pop_before_op(f)?;
                match op {
                    Add => {
                        writeln!(f, "// ADD")?;
                        writeln!(f, "M=D+M")?;
                        _Command::restore_sp(f)?;

                        Ok(())
                    }
                    Sub => {
                        writeln!(f, "// Sub")?;
                        writeln!(f, "M=M-D")?;
                        _Command::restore_sp(f)?;

                        Ok(())
                    }
                    Neg => {
                        writeln!(f, "// Neg")?;
                        writeln!(f, "A=A+1")?;
                        writeln!(f, "M=-D")?;
                        _Command::restore_sp(f)?;

                        Ok(())
                    }
                    And => {
                        writeln!(f, "// And")?;
                        writeln!(f, "M=D&M")?;
                        _Command::restore_sp(f)?;

                        Ok(())
                    }
                    Or => {
                        writeln!(f, "// OR")?;
                        writeln!(f, "M=D|M")?;
                        _Command::restore_sp(f)?;

                        Ok(())
                    }
                    Not => {
                        writeln!(f, "// Not")?;
                        writeln!(f, "A=A+1")?;
                        writeln!(f, "M=!D")?;
                        _Command::restore_sp(f)?;

                        Ok(())
                    }
                    Eq | Lt | Gt => {
                        _Command::compare_and_write(op, &self.label_value, f)?;

                        Ok(())
                    }
                }
            }
            Label(label) => {
                writeln!(f, "({label})")?;

                Ok(())
            }
            GOTO(label) => {
                writeln!(f, "@{label}")?;
                writeln!(f, "0;JMP")?;

                Ok(())
            }
            If(label) => {
                writeln!(f, "@SP")?;
                writeln!(f, "AM=M-1")?;
                writeln!(f, "D=M")?;
                writeln!(f, "@{label}")?;
                writeln!(f, "D;JNE")?;

                Ok(())
            }
            Function { name, args } => {
                writeln!(f, "({name})")?;
                (0..*args).into_iter().try_for_each(|_| {
                    writeln!(f, "D=0")?;
                    _Command::push_from_d_reg(f)?;

                    Ok::<(), fmt::Error>(())
                })?;

                Ok(())
            }
            Call { name, args } => {
                // Pushing return address to stack
                let label = self.call_label.get();

                writeln!(f, "@{name}.ret.{label}")?;
                writeln!(f, "D=A")?;
                writeln!(f, "@SP")?;
                writeln!(f, "AM=M+1")?;
                writeln!(f, "M=D")?;

                _Command::register_to_stack(f, "LCL")?;
                _Command::register_to_stack(f, "ARG")?;
                _Command::register_to_stack(f, "THIS")?;
                _Command::register_to_stack(f, "THAT")?;

                writeln!(f, "@5")?;
                writeln!(f, "D=A")?;
                writeln!(f, "@{}", *args)?;
                writeln!(f, "D=D+A")?;
                writeln!(f, "@SP")?;
                writeln!(f, "D=M-D")?;
                writeln!(f, "@ARG")?;
                writeln!(f, "M=D")?;

                writeln!(f, "@SP")?;
                writeln!(f, "D=M")?;
                writeln!(f, "@LCL")?;
                writeln!(f, "M=D")?;

                writeln!(f, "@{name}")?;
                writeln!(f, "0;JMP")?;

                writeln!(f, "({name}.ret.{label})")?;

                self.call_label.set(label + 1);

                Ok(())
            }
            Return => {
                writeln!(f, "@LCL")?;
                writeln!(f, "D=M")?;
                writeln!(f, "@R15")?; // Frame reg
                writeln!(f, "M=D")?;

                writeln!(f, "@5")?;
                writeln!(f, "D=D-A")?;
                writeln!(f, "@R14")?; // Return address
                writeln!(f, "M=D")?;

                _Command::pop_into_d_reg(f)?;
                writeln!(f, "@ARG")?;
                writeln!(f, "A=M")?;
                writeln!(f, "M=D")?;
                writeln!(f, "@ARG")?;
                writeln!(f, "D=M+1")?;
                writeln!(f, "@SP")?;
                writeln!(f, "M=D")?;

                writeln!(f, "@R15")?;
                writeln!(f, "A=M-1")?;
                writeln!(f, "D=M")?;
                // writeln!(f, "")?;
                writeln!(f, "@THAT")?;
                writeln!(f, "M=D")?;

                ["@NONE_HERE", "@THAT", "@THIS", "@ARG", "@LCL"]
                    .iter()
                    .enumerate()
                    .skip(2)
                    .try_for_each(|(index, addr)| {
                        writeln!(f, "@R15")?;
                        writeln!(f, "D=M")?;
                        writeln!(f, "@{index}")?;
                        writeln!(f, "A=D-A")?;
                        writeln!(f, "D=M")?;
                        writeln!(f, "{addr}")?;
                        writeln!(f, "M=D")?;

                        Ok(())
                    })?;

                writeln!(f, "@R14")?;
                writeln!(f, "A=M")?;
                writeln!(f, "0;JMP")?;

                Ok(())
            }
        }
    }
}

pub struct NotCommandError;

impl fmt::Display for NotCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "supplied string is not a command")
    }
}

impl TryFrom<&str> for _Command {
    type Error = NotCommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            op if ArtithmeticOperation::try_from(op).is_ok() => {
                Ok(_Command::Arithmetic(ArtithmeticOperation::try_from(op)?))
            }
            op if op.starts_with("push") || op.starts_with("pop") => {
                let (command, rest) = op.split_once(" ").expect("Must be ok");
                let (segment, index) = rest.split_once(" ").expect("Must be ok too");

                let segment = Segment::try_from(segment)?;
                let index = index.parse::<u16>().map_err(|_| NotCommandError)?;

                if command == "push" {
                    return Ok(_Command::Push { segment, index });
                } else if command == "pop" {
                    return Ok(_Command::Pop { segment, index });
                } else {
                    unreachable!()
                }
            }
            op if op.starts_with("label") => {
                let (_, label) = op.split_once(" ").expect("Already checked");

                Ok(_Command::Label(label.to_owned()))
            }
            op if op.starts_with("goto") => {
                let (_, label) = op.split_once(" ").expect("already checked");

                Ok(_Command::GOTO(label.to_owned()))
            }
            op if op.starts_with("if-goto") => {
                let (_, label) = op.split_once(" ").expect("Already checked");

                Ok(_Command::If(label.to_owned()))
            }
            op if op.starts_with("function") => {
                let (_, rest) = op.split_once(" ").expect("Must be ok");
                let (name, args) = rest.split_once(" ").expect("Must be ok");
                let name = name.to_owned();
                let args = args.parse().expect("Must be ok");

                Ok(_Command::Function { name, args })
            }
            op if op.starts_with("call") => {
                let mut iterator = op.split(" ").skip(1);
                let name = iterator.next().expect("Must be ok").to_owned();
                let args = iterator
                    .next()
                    .expect("Must be ok")
                    .parse()
                    .expect("Must be ok");

                Ok(_Command::Call { name, args })
            }
            op if op.starts_with("return") => Ok(_Command::Return),
            _ => {
                unimplemented!("{value} is not implemented in parser")
            }
        }
    }
}

#[derive(Debug)]
pub enum ArtithmeticOperation {
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
pub enum Segment {
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
pub(crate) struct Parser<T: BufRead> {
    lines: Lines<T>,
    command: Command,
}

impl<T> Parser<T>
where
    T: BufRead,
{
    pub(crate) fn new(file: T, name: String) -> Self {
        Self {
            // command: Command::Arithmetic(ArtithmeticOperation::Add),
            command: Command::new(name),
            lines: file.lines(),
        }
    }

    pub(crate) fn next_file(&mut self, file: T, name: &str) {
        self.lines = file.lines();
        self.command.name = name.to_owned();
    }

    pub(crate) fn advance(&mut self) -> Option<&Command> {
        // let mut buf: [u8; 1024] = [];
        let mut line = self.lines.next()?.expect("Must be ok");
        while line.trim().starts_with("//") || line.trim().is_empty() {
            line = self.lines.next()?.expect("Must be ok");
        }

        let mut line = line.trim();

        if line.contains("//") {
            let (needed, _) = line.split_once("//").expect("checked in contains");
            line = needed.trim_end();
        }

        self.command.cmd = match _Command::try_from(line) {
            Ok(value) => value,
            Err(_) => return None,
        };

        Some(&self.command)
    }
}
