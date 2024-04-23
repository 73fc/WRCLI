use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::cli::csv::OutputFormat;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(256);
    let header = reader.headers()?.clone();
    for item in reader.records() {
        let record = item?;
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        // println!("{:?}", record);
        ret.push(json_value);
    }
    let contents = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?,
    };
    // let csv_to_json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, contents)?;
    Ok(())
}
