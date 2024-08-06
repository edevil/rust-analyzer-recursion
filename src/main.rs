#![recursion_limit = "256"]

use errors::ServerError;

mod errors;

fn parse_file() -> Result<u64, ServerError> {
    Ok(1)
}

fn main() {
    parse_file().unwrap();
}
