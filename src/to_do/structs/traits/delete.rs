use serde_json::value::Value;
use serde_json::Map;

use crate::generate_path::generate_path;
use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        let path = generate_path();
        write_to_file(&path, state);
        println!("\n\n{} is being deleted\n\n", title)
    }
}
