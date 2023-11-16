use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

use assembler::assembler::Assembler;

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

    let asm = Assembler::new(&file);
    asm.emit(file_path)?;

    Ok(())
}
