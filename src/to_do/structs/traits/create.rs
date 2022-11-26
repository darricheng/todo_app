use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use crate::generate_path::generate_path;
use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        let path = generate_path();
        write_to_file(&path, state);
        println!("\n\n{} is being created\n\n", title);
    }
}
