use clap::Parser;
//wrcli   csv -i input.csv -o output.json -- header -d '.'
use wrcli::{
    base64::Base64SubCommand, process_csv, process_decode, process_encode, process_genpass, Opts,
    SubCommand,
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
            println!("{}", password);
        }
        SubCommand::Base64(subcommd) => match subcommd {
            Base64SubCommand::Encode(opts) => {
                println!("{}", process_encode(&opts.input, opts.format)?);
            }
            Base64SubCommand::Decode(opts) => {
                println!("{}", process_decode(&opts.input, opts.format)?);
            }
        },
    }
    Ok(())
}
