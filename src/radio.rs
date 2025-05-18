use crate::models::{Country, RadioConfig};
use anyhow::{Context, Result};
use std::fs;

pub fn load_radio_data() -> Result<Vec<Country>> {
    let config_path = "stations.json";
    
    let config_content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read {}", config_path))?;
    
    serde_json::from_str::<RadioConfig>(&config_content)
        .with_context(|| format!("Invalid JSON format in {}", config_path))
        .map(|config| config.countries)
}


