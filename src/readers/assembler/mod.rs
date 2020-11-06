//! ## Assembler - Converts clean TOML to `structs`
//! Takes clean and valid user data (provided by the `parser` mod) and organizes/assembles it into `writer` ready structs.

//! ### Main Module API's
//! - `struct WebAPI` - Holds the top-level data for the entirety of the API being made. Mostly config and secret keys.
//! - `struct Endpoint` - Using the user defined parameters, assembles all of the important data into an `actix` struct
