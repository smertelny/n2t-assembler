use std::{
    io::{self, Result},
    path::Path,
};

use compiler::analyzer::Analyzer;

fn main() -> Result<()> {
    let mut args = std::env::args();

    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "no filename passed!"));
    }

    if args.len() > 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "too many arguments"));
    }

    let file_path = args.nth(1).unwrap();
    let file_path: &Path = file_path.as_ref();

    let mut files = Vec::with_capacity(10);
    if file_path.is_file() && file_path.extension().expect("failed to get file extension") == "jack"
    {
        files.push(file_path.to_owned());
    } else if file_path.is_dir() {
        for file in file_path.read_dir().expect("failed to read directory") {
            if let Ok(entry) = file {
                if let Some(extention) = entry.path().extension() {
                    if extention == "jack"
                        && !entry
                            .path()
                            .file_name()
                            .expect("Must have filename")
                            .to_str()
                            .expect("Must convert to str normally")
                            .starts_with(".")
                    {
                        files.push(entry.path());
                    }
                }
            }
        }
    } else {
        // TODO: Need to try simlinks to check if this will be triggered
        panic!("Provided string neigher path nor file");
    }

    Analyzer::process(files.as_slice())

    // Ok(())
}
