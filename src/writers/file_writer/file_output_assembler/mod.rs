mod actix_route_generator;
mod main_method_generator;
mod util_generator;

use crate::readers::assembler::{EndpointGroup, WebAPI};
use util_generator::database_generator::DatabaseInfo;

/// Handles the assembly of the individual final output strings
/// from all of the submodule generators.
///
/// Also we can now tear apart the groups and generate strings `EndpointGroup`-wise
pub struct FileOutputAssembler {
    group: EndpointGroup,
}

impl FileOutputAssembler {
    /// Constructs the assembler
    pub fn from_endpoint_group(group_ref: &EndpointGroup) -> Self {
        let group = (*group_ref).clone();
        FileOutputAssembler { group }
    }

    /// Gets the file ready actix route code that should go in `src/<group_name>/routes.rs`
    pub fn get_actix_routes_string(&self) -> String {
        // total output string to-be
        let mut full_output_string = String::new();

        // the generator object has more than one call that we need, so it's
        // more efficient to build it now instead of making an abstract call
        let route_generator = actix_route_generator::HTTPGetEndpointBuilder::new();

        // append the import header string first
        full_output_string.push_str(&route_generator.get_header_import_string());

        // for each endpoint, write the actix route method code
        for endpoint in &self.group.get_endpoints() {
            let endpoint_string = route_generator.create_endpoint(endpoint);
            full_output_string.push_str(&format!("{}\n", endpoint_string));
        }

        full_output_string
    }

    /// `main.rs` method output generator interface
    pub fn get_main_method_string() -> String {
        main_method_generator::get_main_method_string()
    }

    /// Util method generator interface. Requires extra data for the database
    /// info, so we need the `WebAPI` passed in here
    pub fn get_util_method_string(&self, api_config: &WebAPI) -> String {
        let collection_name = self.group.collection_name.clone();
        let db_info = DatabaseInfo::from_web_api(api_config, collection_name);

        self.build_util_method_string(db_info)
    }

    fn build_util_method_string(&self, db_info: DatabaseInfo) -> String {
        // create a util_method builder and generate the util file string
        // for all of the endpoints at once
        let endpoints = &self.group.get_endpoints();
        let util_builder = util_generator::UtilBuilder::new(db_info);

        util_builder.get_util_method_string(endpoints)
    }
}
