use csv::Reader;
use serde::{Deserialize, Serialize};

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut players = Vec::with_capacity(100);
    for player in reader.deserialize::<Player>() {
        let player = player?;
        players.push(player);
    }

    let json = serde_json::to_string_pretty(&players)?;
    std::fs::write(output, json)?;

    Ok(())
}
