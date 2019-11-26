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
    #[structopt(short ="c", long = "config_file")]
    config_file: Option<String>,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let json: String = std::fs::read_to_string(&args.file).unwrap();
    let openapi: OpenApi3 = serde_json::from_str(&json)?;
    let postman_convert_option: PostmanConvertOption = if args.config_file.is_some() {
        let option: String = std::fs::read_to_string(&args.config_file.unwrap()).unwrap();
        serde_json::from_str(&option)?
    } else {
        PostmanConvertOption::default()
    };
    let postman_collection = to_postman_colletion_2c1(openapi, postman_convert_option);
    let postman_str: String = serde_json::to_string(&postman_collection)?;
    std::fs::write(args.output, postman_str)?;
    Ok(())
}
