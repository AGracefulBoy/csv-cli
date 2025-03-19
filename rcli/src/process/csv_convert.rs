use std::fs;
use std::ptr::write;
use csv::Reader;
use crate::cli::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;

        let value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output,content)?;
    Ok(())
}