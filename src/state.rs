use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use std::path::PathBuf;

pub fn read_file(file_name: &PathBuf) -> Map<String, Value> {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone(); // as_object returns a reference, so we clone to return a Map (not &Map)
    state
}

pub fn write_to_file(file_name: &PathBuf, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to write file");
}
