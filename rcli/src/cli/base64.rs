use clap::Parser;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode a string to base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "decode a string from base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub input: String,

    #[arg(long,value_parser = parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}


#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub input: String,

    #[arg(long,value_parser = parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Unknown format: {}", s)),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
