use clap::{App, Arg, SubCommand, crate_authors, crate_version};
use crate::readers;
use crate::writers::{self, dir_builder::DirectoryBuilder};
use std::path::Path;
use std::process::Command;

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
                .arg(Arg::with_name("filename")
                    .help("The .toml file to read from")
                    .required(true)
                    .index(1)))
        .subcommand(
            SubCommand::with_name("run")
                .about("Execute \"cargo run\" in directory specified in .toml file")
                .arg(Arg::with_name("filename")
                    .help("The .toml file containing the path in which to execute \"cargo run\"")
                    .required(true)
                    .index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let file = matches.value_of("filename").unwrap();
        println!("Building {:?}", file);
        build_api(file).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let file = matches.value_of("filename").unwrap();
        println!("Running {:?}", file);
        run_api(file).unwrap();
    }
}

fn build_api(file_name: &str) -> std::io::Result<()>{
    // read in toml and print it (for debug)
    let toml_reader = readers::parser::InputFileReader::from_file(file_name);

    // make the sub directories with a DirectoryBuilder
    let base_output_dir_str: String = toml_reader.toml_data.path_base.clone();

    // each group will get a subdirectory so we need the names now
    let group_names = toml_reader.toml_data.get_group_names();
    let mut dir_builder = DirectoryBuilder::new(&base_output_dir_str, group_names);
    dir_builder.build().unwrap();

    // writers::file_writer
    writers::file_writer::write(&toml_reader.toml_data, dir_builder).unwrap();

    // format all .rs files in generated directory
    let path_string = String::from("./") + &base_output_dir_str;
    let path = Path::new(&path_string);
    writers::post_operator::do_post_write_ops(&path).unwrap();

    // nice little out message for now (pre-cli lol)
    println!("Done, generated files are at {}.", &base_output_dir_str);
    Ok(())
}

fn run_api(file_name: &str) -> std::io::Result<()> {
    // read in output dir from toml
    let toml_reader = readers::parser::InputFileReader::from_file(file_name);
    let base_output_dir_str: String = toml_reader.toml_data.path_base;
    let path_string = String::from("./") + &base_output_dir_str;
    let path = Path::new(&path_string);

    // cargo run in path
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--manifest-path",
            path.join("Cargo.toml").to_str().unwrap(),
        ])
        .output()?;
    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}