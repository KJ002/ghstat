extern crate chrono;

use serde_json::json;
use chrono::prelude::*;

use std::fs::File;
use std::io::{Write, Read};

fn read_cache() -> Option<serde_json::Value> {
    let mut buffer = String::new();

    let mut file = match File::open("/home/james/.ghstats/cache.json") {
        Ok(handle) => handle,
        Err(_) => return None
    };

    let json_str = match file.read_to_string(&mut buffer) {
        Ok(_) => buffer.as_str(),
        Err(_) => return None
    };

    match serde_json::from_str(json_str) {
        Ok(result) => Some(result),
        Err(_) => None
    }
}

fn get_github_data() -> serde_json::Value {
     let mut result: serde_json::Value = match ureq::get("https://api.github.com/users/KJ002").call() {
        Ok(response) => response.into_json::<serde_json::Value>().unwrap(),
        Err(_) => panic!("Error")
    };

    result["test"] = json!(Local::now().timestamp());
    result
}

fn main() {
    println!("{:?}", read_cache())
}
