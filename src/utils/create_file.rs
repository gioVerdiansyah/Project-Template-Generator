use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn create_file_content<P: AsRef<Path>>(path: P, content: String) -> io::Result<()> {
    let path_ref = path.as_ref();

    let mut file = File::create(path_ref)?;
    file.write_all(content.as_bytes())?;

    println!("Created file: '{:?}'", path_ref.to_path_buf());
    Ok(())
}
