use crate::readers::assembler::WebAPI;
use std::fs::DirBuilder;
use std::path::PathBuf;

const OUTPUT_DIR_NAME: &str = "generated_output";

/// Creates the boilerplate config files as well as the directories (but does not populate them).
// pub fn create_base_files(api_config: &WebAPI) -> Result {
pub fn create_base_files() {
    let builder = DirBuilder::new();
    let mut base_path = PathBuf::from(OUTPUT_DIR_NAME);
    builder.create(&base_path).unwrap();
    let config_paths = ["database", "config"];
    for path in &config_paths {
        base_path.push(path);
        builder.create(&base_path).unwrap();
        base_path.pop();
    }
}
