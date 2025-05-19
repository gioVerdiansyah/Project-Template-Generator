use std::{env, fs, io};
use std::path::Path;

fn main() {
    println!("\n===== RESET main.rs =====");
    let args: Vec<String> = env::args().collect();

    for arg in args {
        if arg == "--development" {
            delete_outer_projects();
        } else if arg == "--with_example" {
            match clear_directory_except_self("src/templates/example") {
                Ok(_) => println!("Deleted \"src/templates/example\""),
                Err(e) => eprintln!("Failed to delete folder \"src/templates/example\"\n{}", e),
            };
        } else if arg.starts_with("--") {
            eprintln!("Unknown or invalid argument: {}", arg);
            eprintln!("Usage: --development [optional] | --with_example [optional]");
            std::process::exit(1);
        }
    }

    let contents_path = Path::new("src/templates/contents");
    if contents_path.exists() {
        match fs::remove_dir_all(contents_path) {
            Ok(_) => println!("Deleted \"{}\"", contents_path.display()),
            Err(e) => eprintln!("Failed to deleted folder \"{}\"\n{}", contents_path.display(), e),
        }
    }

    remove_file("src/templates/create_contents.rs");
    remove_file("src/templates/mod.rs");

    match replace_main_file("src/main.rs"){
        Ok(_) => println!("\n===== SUCCESSFULLY RESET main.rs ====="),
        Err(e) => println!("Error while reset main.rs!\n{}", e)
    };
}

fn replace_main_file<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    let new_content = r#"
fn main() {
  println!("Run bin\\copier_template.rs first!")
}"#.to_string();
    fs::write(file_path, new_content.trim_start())?;
    Ok(())
}

fn remove_file(file_path: &str) {
    let path = Path::new(file_path);

    if path.exists() && path.is_file() {
        match fs::remove_file(path) {
            Ok(_) => println!("Deleted file \"{}\"", path.display()),
            Err(e) => eprintln!("Failed to deleted file \"{}\"\n{}", path.display(), e),
        }
    } else {
        println!("File tidak ditemukan atau bukan file biasa.");
    }
}

fn delete_outer_projects(){
    let current_dir = env::current_dir().expect("Gagal mendapatkan direktori saat ini");

    let exceptions = [
        ".git",
        "src",
        "target",
        ".gitignore",
        "Cargo.lock",
        "Cargo.toml",
        "@LICENSE",
        "README.md",
    ];

    for entry in fs::read_dir(&current_dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if exceptions.contains(&name_str.as_ref()) {
            continue;
        }

        if path.is_dir() {
            match fs::remove_dir_all(&path) {
                Ok(_) => println!("Delted folder \"{:?}\"", path.display()),
                Err(e) => eprintln!("Failed to delte folder \"{:?}\"\n{}", path.display(), e),
            }
        } else {
            match fs::remove_file(&path) {
                Ok(_) => println!("Deleted file \"{:?}\"", path.display()),
                Err(e) => eprintln!("Failed to delete file \"{:?}\"\n{}", path.display(), e),
            }
        }
    }
}

fn clear_directory_except_self<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}