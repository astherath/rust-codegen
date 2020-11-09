//! Writer module that writes most of the generated code in the form of
//! the actual actix endpoint methods.
//!
//! Relies on data from `Endpoint` structs.
//!
//! Should ONLY ever be written to a file AFTER the header string has been
//! written to it (else compile will fail).

use crate::readers::assembler::Endpoint;
