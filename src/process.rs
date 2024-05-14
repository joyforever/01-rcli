use csv::Reader;
use serde_json::Value;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;

    let headers = reader.headers()?.clone();
    let mut records = Vec::with_capacity(128);

    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }

    let json = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
    };
    std::fs::write(output, json)?;

    Ok(())
}
