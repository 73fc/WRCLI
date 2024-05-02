use crate::process_generate;
use crate::process_sign;
use crate::process_verify;
use crate::CmdExector;

use super::{verify_file, verify_path};
use std::{fs, path::PathBuf, str::FromStr};

use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a private/public key")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "Generate key")]
    Generate(TextGnenrateOpts),
}

#[derive(Clone, Copy, Debug, Parser)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Clone, Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    // #[arg(short, long)]
    // pub text: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Clone, Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sign: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Clone, Debug, Parser)]
pub struct TextGnenrateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl std::fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_verify(&self.input, &self.key, self.format, &self.sign)?;
        println!("{:?}", ret);
        Ok(())
    }
}

impl CmdExector for TextGnenrateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.k");
                let file_name = name.clone();
                let _ = fs::write(name, &ret[0]);
                println! {"key is generated on the file {:?}", file_name};
            }
            TextSignFormat::Ed25519 => {
                let name = self.output.join("Ed25519.sk");
                let file_name = name.clone();
                let _ = fs::write(name, &ret[0]);
                println! {"signing key is generated on the file {:?}", file_name};
                let name = self.output.join("Ed25519.pk");
                let file_name = name.clone();
                let _ = fs::write(name, &ret[1]);
                println! {"public key is generated on the file {:?}", file_name};
            }
        }
        Ok(())
    }
}
