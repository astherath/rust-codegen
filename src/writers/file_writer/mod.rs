pub mod body_writer;
pub mod header_writer;
pub mod http_get_writer;
pub mod util_writer;
use crate::readers::assembler::{Endpoint, WebAPI};

pub fn write(api_config: &WebAPI) -> std::io::Result<()> {
    // total output string to-be
    let mut full_output_string = String::new();

    // extract the list of endpoints for ease of borrow
    let endpoints = &api_config.get_all_endpoints();

    // get and concat the header string to the output string
    let header_string = header_writer::HeaderBuilder::get_header_string();
    full_output_string.push_str(&header_string);

    // FIXME: this name is bad; this comment shouldn't be needed
    // writer responsible for writing actix endpoint code
    let writer = http_get_writer::HTTPGetEndpointBuilder::new();
    // for each endpoint, write the actix route method code
    for endpoint in endpoints {
        let endpoint_string = writer.create_endpoint(endpoint);
        full_output_string.push_str(&format!("{}\n", endpoint_string));
    }

    // printing out the route file string
    println!("{}", full_output_string);

    // create a util_method builder and generate the util file string
    // for all of the endpoints at once
    let db_uri = api_config.db_uri.clone();
    let util_builder = util_writer::UtilBuilder::new(db_uri);
    let util_str = util_builder.get_util_method_string(endpoints);

    println!("{}", util_str);

    Ok(())
}
