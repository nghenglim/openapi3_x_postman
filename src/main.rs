use quicli::prelude::*;
use structopt::StructOpt;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate failure;

mod model;
mod postman_model;

use model::OpenApi3;
use postman_model::{to_postman_colletion_2c1, PostmanConvertOption};

#[derive(Debug, StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
    #[structopt(short = "o", long = "output")]
    output: String,
    #[structopt(long = "host")]
    host: String,
    #[structopt(long = "preceding_path", default_value = "")]
    preceding_path: String,
    #[structopt(long = "prepend_tag", default_value = "")]
    prepend_tag: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let json: String = std::fs::read_to_string(&args.file).unwrap();
    let openapi: OpenApi3 = serde_json::from_str(&json)?;
    // std::fs::write("openapi.json", serde_json::to_string(&openapi)?)?;
    let postman_collection = to_postman_colletion_2c1(openapi, PostmanConvertOption {
        host: args.host,
        preceding_path: args.preceding_path,
        prepend_tag: args.prepend_tag,
    });
    let postman_str: String = serde_json::to_string(&postman_collection)?;
    std::fs::write(args.output, postman_str)?;
    Ok(())
}
