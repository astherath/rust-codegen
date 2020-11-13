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
>>>>>>> post_operator format rs WIP
}