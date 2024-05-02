use crate::process_decode;
use crate::process_encode;
use crate::CmdExector;
use std::str::FromStr;

use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode contexts to base64")]
    Encode(Base64Encode),
    #[command(name = "decode", about = "decode contexts from base64")]
    Decode(Base64Decode),
}
#[derive(Clone, Copy, Debug)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
#[derive(Debug, Parser)]
pub struct Base64Encode {
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64Decode {
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

fn parse_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
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

impl CmdExector for Base64Encode {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{}", process_encode(&self.input, self.format)?);
        Ok(())
    }
}

impl CmdExector for Base64Decode {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{}", process_decode(&self.input, self.format)?);
        Ok(())
    }
}
