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

    let mut asm_file;
    if file_path.is_file() {
        let parent_path = file_path.parent().expect("Must be ok");
        let name = file_path.file_stem().expect("Must be ok");
        asm_file = parent_path.join(name);
    } else if file_path.is_dir() {
        let folder_name = file_path.file_name().expect("Must be ok");
        asm_file = file_path.join(folder_name);
    } else {
        // TODO: Need to try simlinks to check if this will be triggered
        panic!("Provided string neigher path nor file");
    }

    asm_file.set_extension("asm");
    let file = BufWriter::new(File::create(asm_file)?);

    translator.emit(file)?;

    Ok(())
}
