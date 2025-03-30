mod csv;
mod genpass;
mod base64;
mod text;

use std::path::{Path, PathBuf};
use clap::Parser;
pub use csv::*;
use crate::cli::genpass::GenPassOpts;
pub use crate::cli::text::TextSubCommand;
pub use self::base64::Base64Format;
pub use self::base64::Base64SubCommand;
pub use self::text::TextSignFormat;

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
    Base64(Base64SubCommand),

    #[command(subcommand,about = "Text sign/verify")]
    Text(TextSubCommand),
}

fn verify_file(filename: &str) -> Result<String,&'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    }else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}