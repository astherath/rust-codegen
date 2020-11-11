use std::path::Path;
use std::process::Command;
mod readers;
mod writers;
fn main() {
    // read in toml and print it (for debug)
    let filename = String::from("sample.toml");
    let toml_reader = readers::parser::InputFileReader::from_file(&filename);
    toml_reader.pretty_print_data();

    // writers::dir_builder
    let base_output_dir_str = String::from("output");
    writers::dir_builder::build(base_output_dir_str).unwrap();

    // writers::file_writer
    // let endpoints = toml_reader.toml_data.get_all_endpoints();
    let file_writer = writers::file_writer::write(&toml_reader.toml_data).unwrap();

    // for path_str in &path_list {
    // let path = Path::new(path_str);
    // run_rustfmt(path).unwrap();
    // }
}

fn _run_rustfmt(path: &Path) -> std::io::Result<()> {
    Command::new("rustfmt").args(path.to_str()).output()?;
    Ok(())
}
