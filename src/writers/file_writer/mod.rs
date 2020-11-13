// mod http_get_writer;
mod file_output_assembler;
use file_output_assembler::FileOutputAssembler;
use crate::readers::assembler::{Endpoint, EndpointGroup, WebAPI};
use crate::writers::dir_builder::{DirectoryBuilder, SubDir};

pub fn write(api_config: &WebAPI, dir_builder: DirectoryBuilder) -> std::io::Result<()> {
    let file_writer = FileOutputAssembler::new();
    for group in &api_config.groups {
        let util_method_string =
            FileWriterAssembler::util_method_string_from_group(api_config, group);
        let util_output_dir = SubDir::Util.as_path_str();
            util_output_dir, util_method_string

        let actix_route_method_string =
            FileWriterAssembler::get_actix_routes_string_for_group(group);
        let actix_output_dir = SubDir::Routes.as_path_str();
        println!(
            "writiting to dir: {:#?} \n{}",
            actix_output_dir, actix_route_method_string
        );

        let main_method_string = main_method_writer::MainMethodBuilder::get_main_method_string();
        let main_method_output_dir = &dir_builder.base_dir;
        println!(
            "writiting to dir: {:#?} \n{}",
            main_method_output_dir, main_method_string
        );
    }
    Ok(())
}

