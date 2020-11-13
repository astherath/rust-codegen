<<<<<<< 4a5223bdd55eee667ff262e6d14613deea392d31
use std::path::Path;
use std::process::Command;
use glob::glob;

fn format_all_rs(path: &Path) -> std::io::Result<()>{
    // Calls rustfmt on all files ending in .rs within path directory and all subdirectories
    let query = path.join("*.rs");
    for entry in glob(query.to_str().unwrap()).expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            Command::new("rustfmt").args(path.to_str()).output()?;
        }
    }
    Ok(())
=======
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
>>>>>>> post_operator format rs WIP
}