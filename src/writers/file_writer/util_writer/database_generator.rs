pub fn get_database_setup_string(db_uri: &String) -> String {
    format!(
        "
            use mongodb::bson::{{doc, document::Document, oid::ObjectId, Bson}};
            use mongodb::{{options::ClientOptions, Client, Collection}};

            pub struct DB {{
                pub client: Client,
                }}

            impl DB {{
                pub async fn init() -> Result<Self> {{
                    let mut client_options = ClientOptions::parse(\"{}\").await?;
                    client_options.app_name = Some(\"TEST\".to_string());
                    Ok(Self {{
                        client: Client::with_options(client_options)?,
                    }})
                }}

                pub fn get_collection(&self, db_name: String, collection: String) -> Collection {{
                    self.client.database(db_name).collection(collection)
                }}

            }}
            ",
        db_uri
    )
}
