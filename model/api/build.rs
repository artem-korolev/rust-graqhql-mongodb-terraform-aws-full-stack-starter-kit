use server::graphql;
use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let schema = graphql::create_schema();
    fs::write("schema.graphql", schema.as_schema_language()).expect("Unable to write file");
    Ok(())
}
