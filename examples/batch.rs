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

    // List of all modules to process
    let modules = vec![
        "absolutebox",
        "cable_route",
        "clutch",
        "demo_shape",
        "display",
        "fixture",
        "item",
        "layout",
        "layout_item",
        "message",
        "metadata",
        "model",
        "section",
        "shape",
        "svg",
        "system",
        "tabular",
        "traits",
    ];

    println!(
        "Generating UML diagrams for {} modules...\n",
        modules.len() + 1
    );

    // Process main project (src)
    let src_dir = runtime_path.join("src");
    let dest = format!("target/doc/runtime");

    if src_dir.exists() {
        println!("Processing main project (runtime)...");
        match rust2uml::src2both(src_dir.to_str().unwrap().to_string(), dest.clone()) {
            Ok(_) => {
                rename_diagram_files(&dest, "runtime");
                println!("  ✓ Generated: {}/runtime.dot", dest);
            }
            Err(e) => eprintln!("  ✗ Error processing main project: {}", e),
        }
    } else {
        eprintln!("  ✗ Main src directory not found: {}", src_dir.display());
    }

    // Process each module
    for module in &modules {
        let module_src_dir = runtime_path.join("src").join(module);
        let dest = format!("target/doc/{}", module);

        if module_src_dir.exists() {
            println!("Processing module: {}...", module);
            match rust2uml::src2both(module_src_dir.to_str().unwrap().to_string(), dest.clone()) {
                Ok(_) => {
                    rename_diagram_files(&dest, module);
                    println!("  ✓ Generated: {}/{}.dot", dest, module);
                }
                Err(e) => eprintln!("  ✗ Error processing {}: {}", module, e),
            }
        } else {
            eprintln!("  ✗ Module not found: {}", module_src_dir.display());
        }
    }

    println!("\nDone! UML diagrams have been generated in target/doc/");

    // Copy to destination if specified
    if should_copy {
        // Calculate destination path relative to runtime path
        let dest_path = runtime_path
            .parent()
            .expect("Runtime path should have a parent directory")
            .join("documentation")
            .join("runtime_architecture")
            .join("module_uml");

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

fn copy_diagrams_to_destination(dest_path: &PathBuf, modules: &[&str]) {
    // Create destination directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(dest_path) {
        eprintln!("  ✗ Error creating destination directory: {}", e);
        return;
    }

    let mut success_count = 0;
    let mut error_count = 0;

    // Copy main project (runtime)
    let source = PathBuf::from("target/doc/runtime");
    let dest = dest_path.join("runtime");

    if source.exists() {
        match copy_directory(&source, &dest) {
            Ok(_) => {
                println!("  ✓ Copied runtime diagrams");
                success_count += 1;
            }
            Err(e) => {
                eprintln!("  ✗ Error copying runtime: {}", e);
                error_count += 1;
            }
        }
    }

    // Copy each module
    for module in modules {
        let source = PathBuf::from(format!("target/doc/{}", module));
        let dest = dest_path.join(module);

        if source.exists() {
            match copy_directory(&source, &dest) {
                Ok(_) => {
                    println!("  ✓ Copied {} diagrams", module);
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("  ✗ Error copying {}: {}", module, e);
                    error_count += 1;
                }
            }
        }
    }

    println!(
        "\nCopy complete: {} succeeded, {} failed",
        success_count, error_count
    );
}

fn copy_directory(src: &PathBuf, dest: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            copy_directory(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
