pub mod header_writer;
pub mod http_get_writer;
use crate::readers::assembler::Endpoint;

pub fn write(endpoints: &Vec<&Endpoint>) -> std::io::Result<()> {
    let writer = http_get_writer::HTTPGetEndpointBuilder::new();
    for endpoint in endpoints {
        let endpoint_string = writer.create_endpoint(endpoint);
        // TODO: placeholder code obviously
        println!("{}", endpoint_string);
    }
    Ok(())
}
