use std::{
    fs::File,
    io::{self, BufWriter, Result, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
};

use compiler::tokenizer::{xml_writer::SimpleXmlWriter, Tokenizer};

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
        // let parent_path = file_path.parent().expect("Must be ok");
        // let name = file_path.file_stem().expect("Must be ok");
        // result_file = parent_path.join(name);
    } else if file_path.is_dir() {
        // let folder_name = file_path.file_name().expect("Must be ok");

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

    files.iter().for_each(|path| {
        let file = File::open(path).expect("failed to open file");
        let mut t = Tokenizer::new(file).expect("failed to create tokenizer");

        let parent_path = path.parent().expect("failed to get parent directory");
        let name = path.file_stem().expect("failed to get file name");
        let name = format!(
            "{}{}",
            name.to_str().expect("failed to convert path to string"),
            "MT"
        );
        let mut output_filename = parent_path.join(name);
        output_filename.set_extension("xml");
        let file = File::create(output_filename).expect("failed to create file");
        let mut buf = BufWriter::new(file);

        buf.write_all(b"<tokens>\n")
            .expect("failed to write into buffer");
        while let Some(token) = t.advance() {
            buf.write_all(b"    ").expect("failed to write into buffer");
            let token = token.expect("failed to get token");
            token
                .write_xml(&mut buf)
                .expect("failed to write into file");
            buf.write_all(b"\n").expect("failed to write into buffer");
            // println!("{:?}", token);
        }
        buf.write_all(b"</tokens>")
            .expect("failed to write into buffer");
    });

    Ok(())
}
