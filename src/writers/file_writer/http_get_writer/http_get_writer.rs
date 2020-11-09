//! Serves as the abstract string-building interface for
//! writing `arctix` GET endpoint code to file.
//!
//! Due to the nature of raw string manipulation/output building,
//! this code tries to hide as much of the actual interface it works with
//! in order to simplify the top-level calls that the `file_writer` mod makes.

/// Main output builder interface (HTTP added in front to avoid naming confusion)
///
/// Uses the exact same template for all HTTP Get endpoints and swaps in the
/// user-specific input vars. with `format!`.
pub struct HTTPGetEndpointBuilder {}

impl HTTPGetEndpointBuilder {}
