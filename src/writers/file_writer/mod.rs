pub mod body_writer;
pub mod header_writer;
pub mod http_get_writer;
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
    println!("{}", full_output_string);

    Ok(())
}
