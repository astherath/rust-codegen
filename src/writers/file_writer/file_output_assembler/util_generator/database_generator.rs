//! This module has all of the actual (large) database string generator code,
//! if DB related code needs to be added/edited this is the first point of contact.

use crate::readers::assembler::WebAPI;

/// Generates a fully contained/functional database system using mongodb.
///
/// Main interface to be used in other generated methods is:
/// `DB::get_collection(db_name, collection)`.
pub fn get_database_setup_string(db_info: &DatabaseInfo) -> String {
    format!(
        "
            pub struct DB {{
                pub client: Client,
                }}

            impl DB {{
                pub async fn init() -> Result<Self, Error> {{
                    let client = Client::with_uri_str(\"{}\")
                        .await
                        .unwrap();
                    Ok(DB {{ client }})
                }}

                pub fn get_collection(&self) -> Collection {{
                    self.client.database(\"{}\").collection(\"{}\")
                }}
            }}
           ",
        &db_info.db_uri, &db_info.db_name, &db_info.collection_name,
    )
}

/// Just returns the import header needed to cover all of the mongodb imports.
pub fn get_database_import_string() -> String {
    String::from(
        "\
        use mongodb::bson::{doc, from_bson, Bson};
        use mongodb::{error::Error, Client, Collection};
        ",
    )
}

/// Super small and simple DB-related struct to hold some variables
pub struct DatabaseInfo {
    db_uri: String,
    db_name: String,
    collection_name: String,
}

impl DatabaseInfo {
    pub fn from_web_api(api_config: &WebAPI, collection_name: String) -> Self {
        let db_uri = api_config.db_uri.clone();
        let db_name = api_config.db_name.clone();
        Self {
            db_uri,
            db_name,
            collection_name,
        }
    }
}
