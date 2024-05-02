use std::path::{Path, PathBuf};

use clap::Parser;

use crate::CmdExector;
pub mod base64;
pub mod csv;
pub mod genpass;
pub mod http;
pub mod text;
#[derive(Debug, Parser)]
#[command(name = "wrcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to other formats")]
    Csv(csv::CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(genpass::GenPassOpts),
    #[command(subcommand)]
    Base64(base64::Base64SubCommand),
    #[command(subcommand)]
    Text(text::TextSubCommand),
    #[command(subcommand)]
    Http(http::HttpSubCommand),
}

impl CmdExector for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(opts) => opts.execute().await,
            SubCommand::Text(opts) => opts.execute().await,
            SubCommand::Http(opts) => opts.execute().await,
        }
    }
}

pub fn verify_file(file_path: &str) -> Result<String, &'static str> {
    if file_path == "-" || std::path::Path::new(file_path).exists() {
        Ok(file_path.into())
    } else {
        Err("File doesn't exist")
    }
}

pub fn verify_path(file_path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(file_path);
    if p.exists() && p.is_dir() {
        Ok(file_path.into())
    } else {
        Err("Path doesn't exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File doesn't exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("no-exist"), Err("File doesn't exist"));
    }
}
