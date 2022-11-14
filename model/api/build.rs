use server::api;
use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let schema = api::create_schema();
    fs::write("schema.graphql", schema.as_schema_language()).expect("Unable to write file");
    Ok(())
}
