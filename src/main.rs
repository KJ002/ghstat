extern crate chrono;

mod args;
mod display;

use chrono::prelude::*;
use serde_json::{json, Value};
use std::{
    fs::File,
    io::{Read, Write},
};
use clap::Parser;
use crate::display::DisplayJson;

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

fn update_cache(user: &str) -> std::io::Result<usize> {
    let mut file: File = File::create("/home/james/.ghstats/cache.json")
        .expect("There was an issue fetching the the file");

    let data = get_github_data(user).to_string();

    file.write(data.as_bytes())
}

fn get_github_data(user: &str) -> serde_json::Value {
    let mut result =
        match ureq::get(format!("https://api.github.com/users/{}", user).as_str()).call() {
            Ok(response) => response.into_json::<serde_json::Value>().unwrap(),
            Err(_) => panic!("Error"),
        };

    result["ghstats_timestamp"] = json!(Local::now().timestamp());

    result
}

fn safe_read(user: &str) -> serde_json::Value {
    match read_cache() {
        Some(x) => x,
        None => {
            update_cache(user).expect("Unable to request api data.");
            read_cache().expect("Unable to parse json.")
        }
    }
}

fn main() {
    let args = args::Args::parse();

    let mut content = safe_read(&args.user);
    let timestamp = content["ghstats_timestamp"].as_i64().unwrap_or_default();

    if Local::now().timestamp() - timestamp >= args.refresh {
        update_cache(&args.user).expect("Unable to request api data.");
        content = safe_read(&args.user)
    }

    content.json_stdout(&args.key)
}
