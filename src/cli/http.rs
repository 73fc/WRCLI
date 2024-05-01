use super::verify_path;
use clap::Parser;
use std::path::PathBuf;
#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a directory over http")]
    Serve(HttpServeOpts),
}

#[derive(Clone, Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long)]
    pub port: u16,
}
