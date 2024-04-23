use clap::Parser;
pub mod base64;
pub mod csv;
pub mod genpass;

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
}

pub fn verify_input_file(file_path: &str) -> Result<String, &'static str> {
    if file_path == "-" || std::path::Path::new(file_path).exists() {
        Ok(file_path.into())
    } else {
        Err("File doesn't exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File doesn't exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("no-exist"), Err("File doesn't exist"));
    }
}
