mod actix_route_generator;
mod header_generator;
mod main_method_generator;
mod util_generator;

use crate::readers::assembler::{Endpoint, EndpointGroup, WebAPI};
use util_writer::database_generator::DatabaseInfo;

/// Handles the assembly of the individual final output strings
/// from all of the submodule generators.
///
/// Also we can now tear apart the groups and generate strings `EndpointGroup`-wise
pub struct FileOutputAssembler {
    group: &'static EndpointGroup,
}

impl FileOutputAssembler {
    /// Constructs the assembler
    pub fn from_endpoint_group(group: &EndpointGroup) -> Self {
        FileOutputAssembler(group)
    }

    /// Gets the file ready actix route code that should go in `src/<group_name>/routes.rs`
    pub fn get_actix_routes_string_for_group(&self) -> String {
        // total output string to-be
        let mut full_output_string = String::new();

        // get and concat the header string to the output string
        let header_string = header_generator::HeaderBuilder::get_header_string();
        full_output_string.push_str(&header_string);

        // for each endpoint, write the actix route method code
        let route_generator = actix_route_generator::HTTPGetEndpointBuilder::new();
        for endpoint in &self.group.get_endpoints() {
            let endpoint_string = route_generator.create_endpoint(endpoint);
            full_output_string.push_str(&format!("{}\n", endpoint_string));
        }

        full_output_string
    }

    pub fn get_util_method_string_for_group(&self, api_config: &WebAPI) -> String {
        let collection_name = self.group.collection_name.clone();
        let db_info = DatabaseInfo::from_web_api(api_config, collection_name);
        let util_file_string = self.build_util_method_string(db_info);

        format!(&util_file_string)
    }

    fn build_util_method_string(&self, db_info: DatabaseInfo) -> String {
        // create a util_method builder and generate the util file string
        // for all of the endpoints at once
        let endpoints = &self.group.get_endpoints();
        let util_builder = util_writer::UtilBuilder::new(db_info);
        let util_str = util_builder.get_util_method_string(endpoints);

        util_str
    }
}
