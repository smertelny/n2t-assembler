use std::fs::File;
use std::io::{BufWriter, Result};
use std::path::Path;

use crate::compilation_engine::CompilationEngine;

pub struct Analyzer;

impl<'a> Analyzer {
    pub fn process<T: AsRef<Path>>(files: &[T]) -> Result<()> {
        files.iter().for_each(|path| {
            println!("\n FILE: {:?}", &path.as_ref());
            let file = File::open(path).expect("failed to open file");
            // let mut t = Tokenizer::new(file).expect("failed to create tokenizer");
            let mut compiler = CompilationEngine::new(file);

            let parent_path = path
                .as_ref()
                .parent()
                .expect("failed to get parent directory");
            let name = path.as_ref().file_stem().expect("failed to get file name");
            let name = format!(
                "{}{}",
                name.to_str().expect("failed to convert path to string"),
                "M"
            );
            let mut output_filename = parent_path.join(name);
            output_filename.set_extension("xml");
            let file = File::create(output_filename).expect("failed to create file");
            let buf = BufWriter::new(file);

            compiler.compile(buf).expect("failed to compile");

            // buf.write_all(b"<tokens>\n")
            //     .expect("failed to write into buffer");
            // while let Some(token) = t.advance() {
            //     buf.write_all(b"    ").expect("failed to write into buffer");
            //     let token = token.expect("failed to get token");
            //     token
            //         .write_xml(&mut buf)
            //         .expect("failed to write into file");
            //     buf.write_all(b"\n").expect("failed to write into buffer");
            //     // println!("{:?}", token);
            // }
            // buf.write_all(b"</tokens>")
            //     .expect("failed to write into buffer");
        });
        Ok(())
    }
}
