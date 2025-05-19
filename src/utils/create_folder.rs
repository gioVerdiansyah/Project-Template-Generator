use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn create_folder<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
    let path_ref = path.as_ref();

    fs::create_dir_all(path_ref)
        .expect(&format!("Error creating folder {:?}", path_ref));

    println!("Created folder: '{:?}'", path_ref.to_path_buf());
    Ok(path_ref.to_path_buf())
}
