mod generate_path;
mod processes;
mod state;
mod to_do;

use processes::process_input;
use serde_json::value::Value;
use serde_json::Map;
use state::read_file;
use std::env;
use to_do::to_do_factory;

use generate_path::generate_path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let path = generate_path();

    let state: Map<String, Value> = read_file(&path);

    let status: String = match &state.get(title) {
        Some(result) => result.to_string().replace('\"', ""),
        None => String::from("pending"),
    };

    let item = to_do_factory(&status, title).expect(&status);

    process_input(item, String::from(command), &state);
}
