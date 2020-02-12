use quicli::prelude::*;
use structopt::StructOpt;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate failure;

mod model;
mod postman_model;

use model::{OpenApi3, to_openapi3, OpenApi3ConvertOption};
use postman_model::{PostmanCollection2c1, to_postman_colletion_2c1, PostmanConvertOption};

#[derive(Debug, StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
    #[structopt(long="pts")]
    postman_to_swagger: bool,
    #[structopt(long="stp")]
    swagger_to_postman: bool,
    #[structopt(short = "o", long = "output")]
    output: String,
    #[structopt(short ="c", long = "config_file")]
    config_file: Option<String>,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let json: String = std::fs::read_to_string(&args.file).unwrap();
    if args.swagger_to_postman {
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
    } else if args.postman_to_swagger {
        let pman: PostmanCollection2c1 = serde_json::from_str(&json)?;
        let openapi3_convert_option: OpenApi3ConvertOption = if args.config_file.is_some() {
            let option: String = std::fs::read_to_string(&args.config_file.unwrap()).unwrap();
            serde_json::from_str(&option)?
        } else {
            OpenApi3ConvertOption::default()
        };
        let swagger_json = to_openapi3(pman.clone(), openapi3_convert_option);
        // println!("{:?}", pman);
        let swagger_str: String = serde_json::to_string_pretty(&swagger_json)?;
        std::fs::write(args.output, swagger_str)?;
    }
    Ok(())
}
