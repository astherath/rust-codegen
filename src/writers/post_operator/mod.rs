use std::path::Path;
use std::fs;
use std::process::Command;
use glob::glob;

fn format_all_rs(path: &Path) -> std::io::Result<()>{
    for entry in glob("*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => Command::new("rustfmt").args(path.to_str()).output()?,
            Err(e) => println!({"{:?}", e}),
        }
    }
    Ok(())
}