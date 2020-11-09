use std::path::Path;
use std::process::Command;
mod readers;
mod writers;
fn main() {
    let filename = String::from("sample.toml");
    let toml_reader = readers::parser::InputFileReader::from_file(&filename);
    toml_reader.pretty_print_data();

    // writers
    let base_output_dir_str = String::from("output");
    // let path_list = writers::dir_builder::build(base_output_dir_str).unwrap();
    writers::dir_builder::build(base_output_dir_str).unwrap();

    // for path_str in &path_list {
    // let path = Path::new(path_str);
    // run_rustfmt(path).unwrap();
    // }
}

fn _run_rustfmt(path: &Path) -> std::io::Result<()> {
    Command::new("rustfmt").args(path.to_str()).output()?;
    Ok(())
}
