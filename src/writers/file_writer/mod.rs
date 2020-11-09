pub mod body_writer;
pub mod header_writer;
pub mod http_get_writer;
pub mod util_writer;
use crate::readers::assembler::Endpoint;

pub fn write(endpoints: &Vec<&Endpoint>) -> std::io::Result<()> {
    let writer = http_get_writer::HTTPGetEndpointBuilder::new();

    let mut full_output_string = String::new();

    // get and concat the header string to the output string
    let header_string = header_writer::HeaderBuilder::get_header_string();
    full_output_string.push_str(&header_string);

    for endpoint in endpoints {
        let endpoint_string = writer.create_endpoint(endpoint);
        full_output_string.push_str(&format!("{}\n", endpoint_string));
    }

    // printing out the route file string
    println!("{}", full_output_string);

    // now handle the util file string/IO
    let db_uri = String::from("test");
    let util_builder = util_writer::UtilBuilder::new(db_uri);
    let endpoint = endpoints.get(0).unwrap(); // XXX: dont use this for long please
    let util_str = util_builder.get_util_method_string(endpoint);

    println!("{}", util_str);

    Ok(())
}
