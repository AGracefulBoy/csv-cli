use super::verify_file;
use super::verify_path;
use anyhow::anyhow;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),

    #[command(about = "Verify a signature with a public /session key")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub input: String,

    #[arg(short,long,value_parser = verify_file)]
    pub key: String,

    #[arg(long,default_value = "blake3",value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub input: String,
    #[arg(short,long,value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long,default_value = "blake3",value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextGenerateOpts {
    #[arg(long,default_value = "blake3",value_parser = parse_text_sign_format)]
    pub input: TextSignFormat,
    #[arg(short,long,value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(input: &str) -> Result<TextSignFormat, anyhow::Error> {
    input.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow!("Unsupported signing format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "blake3",
        }
    }
}