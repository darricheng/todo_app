use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use crate::generate_path::generate_path;
use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        let path = generate_path();
        write_to_file(&path, state);
        println!("\n\n{} is being set to done\n\n", title)
    }
    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        let path = generate_path();
        write_to_file(&path, state);
        println!("\n\n{} is being set to pending\n\n", title)
    }
}
