use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    println!("\n===== COPYING TEMPLATE =====");

    match copy_template() {
        Ok(_) => {
            println!("\n===== SUCCESSFULLY COPYING TEMPLATE =====");

            replace_main_file("src/main.rs").expect("Error while changing main.rs file.");
        },
        Err(e) => {
            eprintln!("Error while processing template: {}", e);
            std::process::exit(1);
        }
    }
}
pub fn copy_template() -> Result<(), Box<dyn std::error::Error>> {
    let template_path = PathBuf::from("src/templates/example");

    // INIT
    fs::create_dir_all("src/templates/contents")?;
    let mut template_mod = File::create("src/templates/mod.rs")?;
    template_mod.write_all(b"pub mod contents;\npub mod create_contents;")?;

    if template_path.is_dir() {
        let mut content_mod = String::new();
        let mut imports = Vec::new();
        let mut create_contents = String::from(
            "use std::io::Error;\n\
             use std::path::PathBuf;\n\
             use crate::utils::create_folder::create_folder;\n"
        );

        // Process the directory recursively
        process_directory(&template_path, &template_path, &mut content_mod, &mut imports)?;

        // Create create_contents.rs file
        create_contents.push_str(&imports.join("\n"));
        create_contents.push_str("\n\npub fn create_contents() -> Result<PathBuf, Error> {\n");
        create_contents.push_str("    let root_path = PathBuf::from(\".\");\n");
        create_contents.push_str("    // Creating folders\n");

        // Process directories again to create folder structure first
        let mut folder_calls = Vec::new();
        let mut file_calls = Vec::new();
        build_create_contents(&template_path, &template_path, &mut folder_calls, &mut file_calls)?;

        for folder_call in folder_calls {
            create_contents.push_str(&format!("    {}?;\n", folder_call));
        }

        create_contents.push_str("\n    // Creating files\n");
        for file_call in file_calls {
            create_contents.push_str(&format!("    {}?;\n", file_call));
        }

        create_contents.push_str("\n    Ok(root_path)\n}");

        // Create the modules and files
        let mut create_content_mod_file = File::create("src/templates/contents/mod.rs")?;
        create_content_mod_file.write_all(content_mod.as_bytes())?;

        let mut create_content_file = File::create("src/templates/create_contents.rs")?;
        create_content_file.write_all(create_contents.as_bytes())?;
    }

    Ok(())
}

fn process_directory(
    base_dir: &Path,
    current_dir: &Path,
    content_mod: &mut String,
    imports: &mut Vec<String>
) -> Result<(), Box<dyn std::error::Error>> {
    // Create contents directory (only the main one, not subdirectories)
    let contents_dir = PathBuf::from("src/templates/contents");
    fs::create_dir_all(&contents_dir)?;

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if path.is_dir() {
            // Recursively process subdirectory
            process_directory(base_dir, &path, content_mod, imports)?;
        } else {
            // Process file
            let mut clean_name = name.to_string();
            if clean_name.starts_with('.') {
                clean_name = clean_name.trim_start_matches('.').to_string();
            }

            // Get relative path for file identification
            let rel_path = path.strip_prefix(base_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();

            // Create a unique module name using relative path
            let path_parts: Vec<&str> = rel_path.split('/').collect();
            let unique_id = if path_parts.len() > 1 {
                let dir_parts = &path_parts[..path_parts.len()-1];
                let dir_name = dir_parts.join("_");
                format!("{}_{}", dir_name, clean_name)
            } else {
                clean_name.clone()
            };

            // Normalize the name without adding "create_" prefix yet
            let normalized_name = unique_id.replace('.', "_").replace('/', "_");

            // Ensure we don't have duplicate "create_" prefixes
            let module_name = if normalized_name.starts_with("create_") {
                normalized_name.clone()
            } else {
                format!("create_{}", normalized_name)
            };

            // Add to module
            content_mod.push_str(&format!("pub mod {};\n", module_name));

            // Add import to create_contents.rs
            imports.push(format!("use crate::templates::contents::{}::{};", module_name, module_name));

            // Create file implementation
            let file_content = fs::read_to_string(&path)
                .unwrap_or_else(|e| {
                    eprintln!("Failed reading file: '{}'\n {}", path.display(), e);
                    String::new()
                });

            let mut impl_content = String::new();
            impl_content.push_str("use std::io;\n");
            impl_content.push_str("use std::path::PathBuf;\n");
            impl_content.push_str("use crate::utils::create_file::create_file_content;\n");
            impl_content.push_str("use crate::utils::global_args::get_args;\n\n");

            // Function with optional path parameter
            impl_content.push_str(&format!(
                "pub fn {}(dir_path: &str) -> io::Result<()> {{\n", module_name
            ));

            impl_content.push_str("    let patterns = get_args();\n");
            impl_content.push_str(&format!("    let content = r#\"{}\"#;\n", file_content.replace("\"#", "\"\\#")));
            impl_content.push_str("    let mut replaced_content = content.to_string();\n\n");

            impl_content.push_str("    // Replace patterns\n");
            impl_content.push_str("    for (key, value) in patterns.iter() {\n");
            impl_content.push_str("        replaced_content = replaced_content.replace(key, &value);\n");
            impl_content.push_str("    }\n\n");

            // For path management
            let parent_folders = path.parent().unwrap()
                .strip_prefix(base_dir)
                .unwrap_or_else(|_| Path::new(""))
                .to_string_lossy()
                .into_owned();

            impl_content.push_str("    // Create target path\n");

            impl_content.push_str("    let file_path = if dir_path.is_empty() {\n");

            // Jika dir_path kosong, gunakan relative path dari template
            if parent_folders.is_empty() {
                impl_content.push_str(&format!(
                    "        PathBuf::from(r\"{}\")\n", name));
            } else {
                impl_content.push_str(&format!(
                    "        PathBuf::from(r\"{}/{}\")\n", parent_folders, name));
            }

            impl_content.push_str("    } else {\n");
            impl_content.push_str(&format!("        PathBuf::from(dir_path).join(r\"{}\")\n", name));
            impl_content.push_str("    };\n\n");

            // Ensure parent directory exists
            impl_content.push_str("    // Ensure parent directory exists\n");
            impl_content.push_str("    if let Some(parent) = file_path.parent() {\n");
            impl_content.push_str("        std::fs::create_dir_all(parent)?;\n");
            impl_content.push_str("    }\n\n");

            impl_content.push_str("    match create_file_content(&file_path, replaced_content) {\n");
            impl_content.push_str("        Ok(_) => println!(\"Created file: '{:?}'\", file_path),\n");
            impl_content.push_str("        Err(e) => eprintln!(\"Error creating file '{:?}' \\n {:?}\", file_path, e),\n");
            impl_content.push_str("    };\n\n");
            impl_content.push_str("    Ok(())\n");
            impl_content.push_str("}\n");

            // Write implementation to file
            let module_file_path = contents_dir.join(format!("{}.rs", module_name));
            let mut module_file = File::create(module_file_path)?;
            module_file.write_all(impl_content.as_bytes())?;
        }
    }

    Ok(())
}

fn build_create_contents(
    base_dir: &Path,
    current_dir: &Path,
    folder_calls: &mut Vec<String>,
    file_calls: &mut Vec<String>
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Get relative path for create_folder call
            let rel_path = path.strip_prefix(base_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();

            if !rel_path.is_empty() {
                folder_calls.push(format!("create_folder(r\"{}\")", rel_path));
            }

            // Recursively process subdirectories
            build_create_contents(base_dir, &path, folder_calls, file_calls)?;
        } else {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let mut clean_name = file_name.to_string();

            if clean_name.starts_with('.') {
                clean_name = clean_name.trim_start_matches('.').to_string();
            }
            // Normalize file name for function call
            if !clean_name.starts_with("create_") {
                clean_name = format!("create_{}", clean_name.replace('.', "_"));
            } else {
                clean_name = clean_name.replace('.', "_");
            }

            // Get relative path for file location
            let rel_path = path.parent().unwrap()
                .strip_prefix(base_dir)
                .unwrap_or_else(|_| Path::new(""))
                .to_string_lossy()
                .into_owned();

            if rel_path.is_empty() {
                file_calls.push(format!("{}(r\"\")", clean_name)); // Fixed typo in empty path case
            } else {
                file_calls.push(format!("{}(r\"{}\")", clean_name, rel_path));
            }
        }
    }

    Ok(())
}

pub fn replace_main_file<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    let new_content = r#"
use std::env;
use crate::utils::global_args::get_args;
use crate::templates::create_contents::create_contents;
use crate::utils::{global_args, parse_args};

mod templates;
mod utils;

fn main() {
    println!("\n===== CREATING TEMPLATE =====");

    match parse_args::parse_args() {
        Some(patterns) => {
            global_args::set_args(&patterns);

            println!("Trying get args:");
            let args = get_args();
            for (k, v) in args {
                println!("  {} -> {}", k, v);
            }

            match create_contents(){
                Ok(_) => println!("\n===== SUCCESSFULLY CREATE TEMPLATE ====="),
                Err(e) => println!("Error while creating template!\n{}", e)
            };
        },
        None => {
            println!("Warning: no argument used!");
            match create_contents(){
                Ok(_) => println!("\n===== SUCCESSFULLY CREATE TEMPLATE ====="),
                Err(e) => println!("Error while creating template!\n{}", e)
            };
        }
    }
}
"#;

    fs::write(file_path, new_content.trim_start())?;
    println!("Run main.rs:");
    println!("  cargo run -- --pattern='{{\"<the_pattern>\": \"replace_pattern\"}}'");
    println!("");
    println!("For Windows CMD:");
    println!("  cargo run -- --pattern=\"{{\\\"<the_pattern>\\\": \\\"replace_pattern\\\"}}\"");
    println!("");
    println!("Alternative simple format:");
    println!("  cargo run -- --pattern=\"{{<the_pattern>: replace_pattern}}\"");
    Ok(())
}