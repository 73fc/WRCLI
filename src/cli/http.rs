use super::verify_path;
use crate::process_http_serve;
use crate::CmdExector;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
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

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        //println!("serving at http://0.0.0.0:{}//{:?}", self.port, self.dir);
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}
