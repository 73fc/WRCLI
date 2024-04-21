use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "wrcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")] // can use into()
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(short = 'H', long, default_value_t = true)]
    // _t doesn't convert data type, must  compare.
    pub header: bool,
}

fn verify_input_file(file_path: &str) -> Result<String, String> {
    if std::path::Path::new(file_path).exists() {
        Ok(file_path.into())
    } else {
        Err("file doesn't exist".into())
    }
}
