#![feature(rustc_private)]
#![feature(box_patterns)]

use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Error! No path configured!".black().on_red());
        eprintln!(
            "{} {}",
            "Usage: ".yellow(),
            "cargo run --example batch -- <path_to_local_runtime_directory> [--copy]".blue()
        );
        std::process::exit(1);
    }

    let runtime_path = PathBuf::from(&args[1]);

    // Parse optional --copy flag
    let should_copy = args.iter().any(|arg| arg == "--copy");

    if !runtime_path.exists() {
        eprintln!("Error: Path does not exist: {}", runtime_path.display());
        std::process::exit(1);
    }

    let config = rust2uml::Config::default();
    rust2uml::Config::set_global(config);

    // Process main project (src)
    let src_dir = runtime_path.join("src");
    let dest = format!("target/doc/runtime");

    let mut modules: Vec<String> = vec![];

    if src_dir.exists() {
        println!("Processing main project (runtime)...");
        match rust2uml::src2both(src_dir.to_str().unwrap().to_string(), dest.clone()) {
            Ok(_) => {
                rename_diagram_files(&dest, "runtime");
                println!("  ✓ Generated: {}/runtime.dot", dest);
            }
            Err(e) => eprintln!("  ✗ Error processing main project: '{}'", e),
        }

        println!("Collecting list of modules...");

        match fs::read_dir(src_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(dir_entry) => match dir_entry.file_type() {
                            Ok(file_type) => {
                                if file_type.is_dir() {
                                    modules
                                        .push(dir_entry.file_name().to_string_lossy().to_string());
                                }
                            }
                            Err(e) => {
                                eprintln!("  ✗ Error reading file type: '{}'", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("  ✗ Error reading directory entry: '{}'", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("  ✗ Error reading directory: '{}'", e);
            }
        }
    } else {
        eprintln!("  ✗ Main src directory not found: '{}'", src_dir.display());
    }

    println!(
        "Generating UML diagrams for {} modules...\n",
        modules.len() + 1
    );

    // Process each module
    for module in &modules {
        let module_src_dir = runtime_path.join("src").join(module.as_str());
        let dest = format!("target/doc/{}", module);

        if module_src_dir.exists() {
            println!("Processing module: {}...", module);
            match rust2uml::src2both(module_src_dir.to_str().unwrap().to_string(), dest.clone()) {
                Ok(_) => {
                    rename_diagram_files(&dest, module);
                    println!("  ✓ Generated: {}/{}.dot", dest, module);
                    println!("  ✓ Generated: {}/{}.svg", dest, module);
                }
                Err(e) => eprintln!("  ✗ Error processing {}: '{}'", module, e),
            }
        } else {
            eprintln!("  ✗ Module not found: '{}'", module_src_dir.display());
        }
    }

    let success_message = format!("\nDone! UML diagrams have been generated in target/doc/");
    println!("{}", success_message.green());

    // Copy to destination if specified
    if should_copy {
        // Calculate destination path relative to runtime path
        let dest_path = runtime_path
            .parent()
            .expect("Runtime path should have a parent directory")
            .join("runtime")
            .join("diagram");

        println!("\nCopying diagrams to {}...", dest_path.display());
        copy_diagrams_to_destination(&dest_path, &modules);
    }
}

fn rename_diagram_files(dest_dir: &str, module_name: &str) {
    let dest_path = PathBuf::from(dest_dir);

    // Rename ml.dot to {module_name}.dot
    let old_dot = dest_path.join("ml.dot");
    let new_dot = dest_path.join(format!("{}.dot", module_name));
    if old_dot.exists() {
        let _ = fs::rename(&old_dot, &new_dot);
    }

    // Rename ml.svg to {module_name}.svg
    let old_svg = dest_path.join("ml.svg");
    let new_svg = dest_path.join(format!("{}.svg", module_name));
    if old_svg.exists() {
        let _ = fs::rename(&old_svg, &new_svg);
    }
}

fn copy_diagrams_to_destination(dest_path: &PathBuf, modules: &[String]) {
    // Create destination directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(dest_path) {
        eprintln!("  ✗ Error creating destination directory: '{}'", e);
        return;
    }

    let mut success_count = 0;
    let mut error_count = 0;

    // Copy main project (runtime)
    let source = PathBuf::from("target/doc/runtime");

    if source.exists() {
        match copy_directory(&source, dest_path) {
            Ok(_) => {
                println!("  ✓ Copied runtime diagrams");
                success_count += 1;
            }
            Err(e) => {
                eprintln!("  ✗ Error copying runtime: '{}'", e);
                error_count += 1;
            }
        }
    }

    // Copy each module
    for module in modules {
        let source = PathBuf::from(format!("target/doc/{}", module));

        if source.exists() {
            match copy_directory(&source, dest_path) {
                Ok(_) => {
                    println!("  ✓ Copied '{}' diagrams", module);
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("  ✗ Error copying {}: '{}'", module, e);
                    error_count += 1;
                }
            }
        }
    }

    let success_message = format!(
        "\nCopy complete: {} succeeded, {} failed\n",
        success_count, error_count
    );
    println!("{}", success_message.green());
}

fn copy_directory(src: &PathBuf, dest: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();

        if src_path.is_file() && src_path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let dest_path = dest.join(entry.file_name());
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
