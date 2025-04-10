use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_genpass, process_text_key_generate, process_text_sign, process_text_verifier, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
            Ok(())
        }
        SubCommand::GenPass(opts) => {
            let a = process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbol).expect("TODO: panic message");
            println!("{}", a);
            Ok(())
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opt) => {
                process_encode(&opt.input, opt.format)
            }
            Base64SubCommand::Decode(opt) => {
                process_decode(&opt.input, opt.format)
            }
        }

        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opt) => {
                let mut reader: Box<dyn Read> = if opt.input == "-" {
                    Box::new(std::io::stdin())
                } else {
                    Box::new(BufReader::new(File::open(&opt.input)?))
                };

                let key_data = std::fs::read(&opt.key)?;

                let signature = match opt.format {
                    TextSignFormat::Blake3 => process_text_sign(&mut reader, &key_data, opt.format)?,
                    TextSignFormat::Ed25519 => process_text_sign(&mut reader, &key_data, opt.format)?,
                };

                let sig_base64 = BASE64_URL_SAFE_NO_PAD.encode(&signature);
                println!("{}", sig_base64);
                Ok(())
            }
            TextSubCommand::Verify(opt) => {
                let mut reader: Box<dyn Read> = if opt.input == "-" {
                    Box::new(std::io::stdin())
                } else {
                    Box::new(BufReader::new(File::open(&opt.input)?))
                };

                let key_data = std::fs::read(&opt.key)?;
                let sig_data = BASE64_URL_SAFE_NO_PAD.decode(&opt.sig)?;

                let result = match opt.format {
                    TextSignFormat::Blake3 => process_text_verifier(&mut reader, &key_data, &sig_data, opt.format)?,
                    TextSignFormat::Ed25519 => process_text_verifier(&mut reader, &key_data, &sig_data, opt.format)?,
                };

                println!("{}", if result { "Valid signature" } else { "Invalid signature" });
                Ok(())
            }
            TextSubCommand::Generate(opt) => {
                let keys = process_text_key_generate(opt.input)?;
                // TODO: Save keys to output_path
                println!("Keys generated: {:?}", keys);
                Ok(())
            }
        }

        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(subcmd) => {
                println!("subcmd {:?}", subcmd);
                Ok(())
            }
        }
    }
}