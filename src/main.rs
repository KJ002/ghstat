extern crate chrono;

use chrono::prelude::*;

use serde_json::{json, Value};

use std::fs::File;
use std::io::{Read, Write};

fn read_cache() -> Option<serde_json::Value> {
    let mut buffer = String::new();

    let mut file = match File::open("/home/james/.ghstats/cache.json") {
        Ok(handle) => handle,
        Err(_) => return None,
    };

    let json_str = match file.read_to_string(&mut buffer) {
        Ok(_) => buffer.as_str(),
        Err(_) => return None,
    };

    match serde_json::from_str(json_str) {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

fn update_cache() -> std::io::Result<usize> {
    let mut file: File = File::create("/home/james/.ghstats/cache.json")
        .expect("There was an issue fetching the the file");

    let data = get_github_data().to_string();

    file.write(data.as_bytes())
}

fn get_github_data() -> serde_json::Value {
    let mut result = match ureq::get("https://api.github.com/users/KJ002").call() {
        Ok(response) => response.into_json::<serde_json::Value>().unwrap(),
        Err(_) => panic!("Error"),
    };

    result["ghstats_timestamp"] = json!(Local::now().timestamp());

    result
}

fn safe_read() -> serde_json::Value {
    match read_cache() {
        Some(x) => x,
        None => {
            update_cache().expect("Unable to request api data.");
            read_cache().expect("Unable to parse json.")
        }
    }
}

fn json_stdout(key: &str, json: &serde_json::Value) {
    fn operations(value: &Value) {
        match value {
            Value::Null => println!("Null"),
            Value::Bool(x) => println!("{}", x),
            Value::Number(x) => println!("{}", x),
            Value::String(x) => println!("{}", x),
            Value::Array(x) => x.iter().map(operations).collect::<()>(),
            Value::Object(x) => x.values().map(operations).collect::<()>()
        };
    }
    operations(&json[key])
}

fn main() {

    let mut content = safe_read();

    let timestamp = content["ghstats_timestamp"].as_i64().unwrap_or_default();

    if Local::now().timestamp() - timestamp >= 60 {
        update_cache().expect("Unable to request api data.");
        content = safe_read()
    }

    json_stdout("test", &content)
}
