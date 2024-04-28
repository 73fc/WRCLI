use std::fs;

use clap::Parser;
//wrcli   csv -i input.csv -o output.json -- header -d '.'
use wrcli::{
    base64::Base64SubCommand, process_csv, process_decode, process_encode, process_generate,
    process_genpass, process_sign, process_verify, text::TextSubCommand, Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(s) = opts.output {
                s.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            print!("{}", password);
        }
        SubCommand::Base64(subcommd) => match subcommd {
            Base64SubCommand::Encode(opts) => {
                println!("{}", process_encode(&opts.input, opts.format)?);
            }
            Base64SubCommand::Decode(opts) => {
                println!("{}", process_decode(&opts.input, opts.format)?);
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubCommand::Sign(opts) => {
                //println!("the command is : {:?}", opts);
                let signed = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                println!("the command is : {:?}", opts);
                let ret = process_verify(&opts.input, &opts.key, opts.format, &opts.sign)?;
                println!("{:?}", ret);
            }
            TextSubCommand::Generate(opts) => {
                println!("the command is : {:?}", opts);
                let ret = process_generate(opts.format)?;
                match opts.format {
                    wrcli::text::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.k");
                        let file_name = name.clone();
                        let _ = fs::write(name, &ret[0]);
                        println! {"key is generated on the file {:?}", file_name};
                    }
                    wrcli::text::TextSignFormat::Ed25519 => {
                        let name = opts.output.join("Ed25519.sk");
                        let file_name = name.clone();
                        let _ = fs::write(name, &ret[0]);
                        println! {"signing key is generated on the file {:?}", file_name};
                        let name = opts.output.join("Ed25519.pk");
                        let file_name = name.clone();
                        let _ = fs::write(name, &ret[1]);
                        println! {"public key is generated on the file {:?}", file_name};
                    }
                }
            }
        },
    }
    Ok(())
}
