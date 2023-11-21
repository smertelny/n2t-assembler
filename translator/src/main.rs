use std::{env, io, path::Path};

use translator::translator::Translator;

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
    let file_name = file_path.file_stem().expect("No file provided");

    let translator = Translator::new(file_path)?;

    Ok(())
}
