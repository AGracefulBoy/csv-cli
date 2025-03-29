use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts, SubCommand};

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
            },
            Base64SubCommand::Decode(opt) => {
                process_decode(&opt.input, opt.format)
            }
        }
    }
}