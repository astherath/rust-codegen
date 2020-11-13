// mod http_get_writer;
mod file_output_assembler;
use crate::readers::assembler::{Endpoint, EndpointGroup, WebAPI};
use crate::writers::dir_builder::{DirectoryBuilder, SubDir};
use file_output_assembler::FileOutputAssembler;

pub fn write(api_config: &WebAPI, dir_builder: DirectoryBuilder) -> std::io::Result<()> {
    for group in &api_config.groups {
        let output_assembler = FileOutputAssembler::from_endpoint_group(group);

        let util_method_string = output_assembler.get_util_method_string(api_config);
        let util_output_dir = SubDir::Util.as_path_str();
        println!(
            "writing to dir: {:#?} \n{}",
            util_output_dir, util_method_string
        );

        let actix_route_method_string = output_assembler.get_actix_routes_string();
        let actix_output_dir = SubDir::Routes.as_path_str();
        println!(
            "writiting to dir: {:#?} \n{}",
            actix_output_dir, actix_route_method_string
        );

        let main_method_string = output_assembler.get_main_method_string();
        let main_method_output_dir = &dir_builder.base_dir;
        println!(
            "writiting to dir: {:#?} \n{}",
            main_method_output_dir, main_method_string
        );
    }
    Ok(())
}
