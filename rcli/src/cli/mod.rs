mod csv;
mod genpass;
mod base64;

use std::path::Path;
use clap::Parser;
pub use csv::*;
use crate::cli::genpass::GenPassOpts;

pub use self::base64::Base64Format;
pub use self::base64::Base64SubCommand;

#[derive(Debug, Parser)]
#[command(name = "rcli",version,author,about,long_about= None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass",about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand,about = "Base64 encode/decode")]
    Base64(Base64SubCommand)
}

fn verify_file(filename: &str) -> Result<String,&'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    }else {
        Err("File does not exist")
    }
}