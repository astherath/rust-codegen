mod readers;
mod writers;
fn main() {
    let filename = String::from("sample.toml");
    let toml_reader = readers::parser::InputFileReader::from_file(&filename);
    toml_reader.pretty_print_data();

    // writers
    writers::config::create_base_files();
}
