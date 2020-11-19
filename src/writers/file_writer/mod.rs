// mod http_get_writer;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
mod file_output_assembler;
use crate::readers::assembler::{EndpointGroup, WebAPI};
use crate::writers::dir_builder::{DirectoryBuilder, SubDir};
use file_output_assembler::{main_method_generator::MainMethodBuilder, FileOutputAssembler};

pub fn write(api_config: &WebAPI, dir_builder: DirectoryBuilder) -> std::io::Result<()> {
    let root_dir = dir_builder.base_dir;
    let mut file_writer = FileWriter::from_base_dir(root_dir);

    // write the main file (outside of group)
    let mut main_method_string_builder = MainMethodBuilder::new();

    // write to file
    for group in &api_config.groups {
        // update the file directory for the group
        file_writer.update_base_dir(group);

        // instanciate the single output assembler for the group
        let output_assembler = FileOutputAssembler::from_endpoint_group(group);

        // write the util to file
        let util_method_string = output_assembler.get_util_method_string(api_config);
        file_writer.write_output_to_file(&SubDir::Util, util_method_string)?;

        // write the actix route code
        let actix_route_method_string = output_assembler.get_actix_routes_string();
        file_writer.write_output_to_file(&SubDir::Routes, actix_route_method_string)?;

        // add the group data to writer so the main function can mount the routes
        main_method_string_builder.mount_group(group);

        // finally write the mod file for the group
        file_writer.write_mod_rs_to_file().unwrap();
    }

    // write fully assembled main file
    let main_function_string = main_method_string_builder.get_main_method_string();
    file_writer.write_main_function(main_function_string)?;

    Ok(())
}

/// Only handled file IO based on sub directories
struct FileWriter {
    base_dir: PathBuf,
    group_base_dir: PathBuf,
}

impl FileWriter {
    fn from_base_dir(base_dir: PathBuf) -> Self {
        let group_base_dir = base_dir.clone();
        Self {
            base_dir,
            group_base_dir,
        }
    }

    /// Private helper function that handles file IO given the sub/base dir
    fn write_output_to_file(&mut self, sub_dir: &SubDir, content: String) -> std::io::Result<()> {
        // create new directory with the sub_dir and the filename
        let mut file_path = PathBuf::from(sub_dir.as_path_str());
        let filename = String::from("mod.rs");

        // add file to base path
        file_path.push(filename);
        self.group_base_dir.push(file_path);

        // open file and write `content` to it
        let mut file = File::create(&self.group_base_dir)?;

        // pop the subdir from the base path once file was made
        // NOTE: this is a VERY ugly way to do this, but it ~should~ always work.
        // probably time to find a better way to do this eventually
        self.group_base_dir.pop();
        self.group_base_dir.pop();

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

    /// Changes the base directory to be based on the group
    fn update_base_dir(&mut self, group: &EndpointGroup) {
        self.group_base_dir = self.base_dir.clone();
        self.group_base_dir.push(&group.name);
    }

    /// Generate the static `mod.rs` string
    fn get_mod_string(&self) -> String {
        "\npub mod util;
        pub mod routes;\n"
            .to_string()
    }

    /// Doesn't require a path OR a string (same modules get written to always)
    fn write_mod_rs_to_file(&mut self) -> std::io::Result<()> {
        // create new directory with the sub_dir and the filename
        let filename = String::from("mod.rs");

        // add file to base path
        self.group_base_dir.push(filename);

        // open file and write `content` to it
        let mut file = File::create(&self.group_base_dir)?;
        let mod_file_content = self.get_mod_string();
        file.write_all(mod_file_content.as_bytes())?;

        self.group_base_dir.pop();

        Ok(())
    }
}
