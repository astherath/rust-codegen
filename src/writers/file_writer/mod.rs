mod body_writer;
mod header_writer;
mod http_get_writer;
mod main_method_writer;
mod util_writer;
use crate::readers::assembler::{Endpoint, EndpointGroup, WebAPI};
use crate::writers::dir_builder::{DirectoryBuilder, SubDir};
use util_writer::database_generator::DatabaseInfo;

use std::collections::HashMap;

pub fn write(api_config: &WebAPI, dir_builder: DirectoryBuilder) -> std::io::Result<()> {
    let mut sub_dir_map = HashMap::new();
    for sub_dir in &dir_builder.sub_dirs {
        sub_dir_map.insert(sub_dir.clone(), sub_dir.as_path_str());
    }

    for group in &api_config.groups {
        let util_method_string =
            FileWriterAssembler::util_method_string_from_group(api_config, group);
        let util_output_dir = sub_dir_map.get(&SubDir::Util);
        println!(
            "writiting to dir: {:#?} \n{}",
            util_output_dir, util_method_string
        );

        let actix_route_method_string =
            FileWriterAssembler::get_actix_routes_string_for_group(group);
        let actix_output_dir = sub_dir_map.get(&SubDir::Routes);
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

/// Holds the actual implementation of the `write()` implemented.
///
/// We really only need some of the `WebAPI` data, so it's just easier to work
/// with the data like this.
///
/// Also we can now tear apart the groups and generate strings `EndpointGroup`-wise
struct FileWriterAssembler;

impl FileWriterAssembler {
    /// Gets the file ready actix route code that should go in `src/<group_name>/routes.rs`
    fn get_actix_routes_string_for_group(group: &EndpointGroup) -> String {
        // total output string to-be
        let mut full_output_string = String::new();

        // get and concat the header string to the output string
        let header_string = header_writer::HeaderBuilder::get_header_string();
        full_output_string.push_str(&header_string);

        // FIXME: this name is bad; this comment shouldn't be needed
        // writer responsible for writing actix endpoint code
        let writer = http_get_writer::HTTPGetEndpointBuilder::new();

        // for each endpoint, write the actix route method code
        for endpoint in &group.get_endpoints() {
            let endpoint_string = writer.create_endpoint(endpoint);
            full_output_string.push_str(&format!("{}\n", endpoint_string));
        }

        full_output_string
    }

    fn util_method_string_from_group(api_config: &WebAPI, group: &EndpointGroup) -> String {
        let collection_name = group.collection_name.clone();
        let db_info = DatabaseInfo::from_web_api(api_config, collection_name);
        let util_file_string = Self::get_util_method_string_for_group(db_info, group);

        format!(
            "util file string for group with name {}:\n{}",
            &group.name, &util_file_string
        )
    }

    fn get_util_method_string_for_group(db_info: DatabaseInfo, group: &EndpointGroup) -> String {
        let endpoints = &group.get_endpoints();

        // create a util_method builder and generate the util file string
        // for all of the endpoints at once
        let util_builder = util_writer::UtilBuilder::new(db_info);
        let util_str = util_builder.get_util_method_string(endpoints);

        util_str
    }
}
