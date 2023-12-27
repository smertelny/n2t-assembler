use std::{
    env,
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

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

    let mut translator = Translator::new(file_path)?;

    let parent_path = file_path.parent().expect("Must be ok");
    let name = file_path.file_stem().expect("Must be ok");
    let mut asm_file = parent_path.join(name);
    asm_file.set_extension("asm");
    let file = BufWriter::new(File::create(asm_file)?);

    translator.emit(file)?;

    Ok(())
}
