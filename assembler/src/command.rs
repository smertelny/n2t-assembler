#[derive(Debug, PartialEq)]
pub enum Command<'a> {
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

#[cfg(test)]
mod tests {
    use crate::command::*;

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
