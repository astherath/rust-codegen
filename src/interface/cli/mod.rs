use clap::{App, Arg, SubCommand, crate_authors, crate_version};
use crate::readers;
use crate::writers;
use crate::writers::dir_builder::DirectoryBuilder;

//<cli-name> run <generated-output-folder-name>
//<cli-name> build <generated-output-folder-name>
pub fn parse_args() {
    let app_name = "apigen";
    let about = "Generates directories and skeletons for APIs from .toml input file";
    let matches = App::new(app_name)
        .author(crate_authors!())
        .version(crate_version!())
        .about(about)
        .subcommand(
            SubCommand::with_name("build")
                .about("Generate directory and associated files from .toml")
                .arg(Arg::with_name("file")
                    .help("The .toml file to read from")
                    .required(true)
                    .index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let file = matches.value_of("file").unwrap();
        println!("Building {:?}", file);
        build_api(file);
    }
}

fn build_api(file: &str) {
    // read in toml and print it (for debug)
    let filename = String::from(file);
    let toml_reader = readers::parser::InputFileReader::from_file(&filename);

    // make the sub directories with a DirectoryBuilder
    let base_output_dir_str = String::from("GENERATED");

    // each group will get a subdirectory so we need the names now
    let group_names = toml_reader.toml_data.get_group_names();
    // let mut dir_builder = DirectoryBuilder::new(base_output_dir_str.clone(), group_names);
    let mut dir_builder = DirectoryBuilder::new(&base_output_dir_str, group_names);
    dir_builder.build().unwrap();

    // writers::file_writer
    writers::file_writer::write(&toml_reader.toml_data, dir_builder).unwrap();

    // format all .rs files in generated directory
    writers::post_operator::do_post_write_ops(&String::from("./GENERATED")).unwrap();

    // nice little out message for now (pre-cli lol)
    println!("Done, generated files are at {}.", &base_output_dir_str)
}