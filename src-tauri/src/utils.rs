use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;

pub fn load_file(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("Failed to load file {}: {error}", path.display()))
}

pub fn load_json_file<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    parse_json(path, &load_file(path)?)
}

pub fn load_yaml_file<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    parse_yaml(path, &load_file(path)?)
}

pub fn parse_json<T: DeserializeOwned>(path: &Path, data: &[u8]) -> Result<T, String> {
    serde_json::from_slice(&data)
        .map_err(|error| format!("Failed to parse JSON for {}: {error}", path.display()))
}

pub fn parse_yaml<T: DeserializeOwned>(path: &Path, data: &[u8]) -> Result<T, String> {
    serde_yaml::from_slice(&data)
        .map_err(|error| format!("Failed to parse YAML for {}: {error}", path.display()))
}
