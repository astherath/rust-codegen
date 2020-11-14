// mod http_get_writer;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
mod file_output_assembler;
use crate::readers::assembler::{Endpoint, EndpointGroup, WebAPI};
use crate::writers::dir_builder::{DirectoryBuilder, SubDir};
use file_output_assembler::FileOutputAssembler;

pub fn write(api_config: &WebAPI, dir_builder: DirectoryBuilder) -> std::io::Result<()> {
    let mut base_dir = dir_builder.base_dir.clone();
    let mut file_writer = FileWriter::from_base_dir(base_dir);

    // write to file
    for group in &api_config.groups {
        let output_assembler = FileOutputAssembler::from_endpoint_group(group);

        let util_method_string = output_assembler.get_util_method_string(api_config);
        file_writer.write_output_to_file(&SubDir::Util, util_method_string)?;

        let actix_route_method_string = output_assembler.get_actix_routes_string();
        file_writer.write_output_to_file(&SubDir::Routes, actix_route_method_string)?;

        let main_method_string = output_assembler.get_main_method_string();
        file_writer.write_main_function(main_method_string);
    }
    Ok(())
}

/// Only handled file IO based on sub directories
struct FileWriter {
    base_dir: PathBuf,
}

impl FileWriter {
    fn from_base_dir(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Private helper function that handles file IO given the sub/base dir
    fn write_output_to_file(&mut self, sub_dir: &SubDir, content: String) -> std::io::Result<()> {
        // create new directory with the sub_dir and the filename
        let mut file_path = PathBuf::from(sub_dir.as_path_str());
        let filename = String::from("mod.rs");

        // add file to base path
        file_path.push(filename);
        self.base_dir.push(file_path);

        // open file and write `content` to it
        let mut file = File::create(&self.base_dir)?;

        // pop the subdir from the base path once file was made
        // NOTE: this is a VERY ugly way to do this, but it ~should~ always work.
        // probably time to find a better way to do this eventually
        self.base_dir.pop();
        self.base_dir.pop();

        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Special main function writer (no subdir)
    fn write_main_function(&mut self, content: String) -> std::io::Result<()> {
        // create new directory with the sub_dir and the filename
        let filename = String::from("main.rs");

        // add file to base path
        self.base_dir.push(filename);

        // open file and write `content` to it
        let mut file = File::create(&self.base_dir)?;
        file.write_all(content.as_bytes())?;

        self.base_dir.pop();

        Ok(())
    }
}
