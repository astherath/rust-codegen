mod readers;
mod writers;
use writers::dir_builder::DirectoryBuilder;
fn main() {
    // read in toml and print it (for debug)
    let filename = String::from("sample.toml");
    let toml_reader = readers::parser::InputFileReader::from_file(&filename);

    // make the sub directories with a DirectoryBuilder
    let base_output_dir_str = String::from("GENERATED/src");

    // each group will get a subdirectory so we need the names now
    let group_names = toml_reader.toml_data.get_group_names();
    let mut dir_builder = DirectoryBuilder::new(base_output_dir_str.clone(), group_names);
    dir_builder.build().unwrap();

    // writers::file_writer
    writers::file_writer::write(&toml_reader.toml_data, dir_builder).unwrap();

    // nice little out message for now (pre-cli lol)
    println!("Done, generated files are at {}.", &base_output_dir_str)
}
