mod cli;
mod process;
pub use cli::{base64, Opts, SubCommand};
pub use process::b64::process_decode;
pub use process::b64::process_encode;
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_genpass;
