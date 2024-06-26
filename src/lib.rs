mod cli;
mod process;
mod utils;
pub use cli::base64::*;
pub use cli::http::*;
pub use cli::text::*;
pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::b64::process_decode;
pub use process::b64::process_encode;
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_genpass;
pub use process::http_serve::process_http_serve;
pub use process::text::{process_generate, process_sign, process_verify};
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
