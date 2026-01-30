#![feature(rustc_private)]
#![feature(box_patterns)]

use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example batch -- <path_to_runtime_directory>");
        eprintln!(
            "Example: cargo run --example batch -- /Users/colin.matthews/repos/bacapp/runtime"
        );
        std::process::exit(1);
    }

    let runtime_path = PathBuf::from(&args[1]);

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
            Ok(_) => println!("  ✓ Generated: {}/ml.dot", dest),
            Err(e) => eprintln!("  ✗ Error processing main project: {}", e),
        }
    } else {
        eprintln!("  ✗ Main src directory not found: {}", src_dir.display());
    }

    // Process each module
    for module in modules {
        let module_src_dir = runtime_path.join("src").join(module);
        let dest = format!("target/doc/{}", module);

        if module_src_dir.exists() {
            println!("Processing module: {}...", module);
            match rust2uml::src2both(module_src_dir.to_str().unwrap().to_string(), dest.clone()) {
                Ok(_) => println!("  ✓ Generated: {}/ml.dot", dest),
                Err(e) => eprintln!("  ✗ Error processing {}: {}", module, e),
            }
        } else {
            eprintln!("  ✗ Module not found: {}", module_src_dir.display());
        }
    }

    println!("\nDone! UML diagrams have been generated in target/doc/");
}
