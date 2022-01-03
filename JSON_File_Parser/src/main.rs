use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn main() -> std::io::Result<()> {
    println!("{:?}", std::env::current_dir().unwrap());
    let mut p = Path::new("./files/sample.json");
    let mut file = File::open(p.as_os_str())?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let json_converted: Value = serde_json::from_str(&text)?;
    println!("Reading completed! {}", &text[0..100]);
    println!("name: {}, question q1: {}", json_converted["name"], json_converted["quiz"]["sport"]["q1"]["question"]);

    let address = Address {
        street: "40/7 Electronic City \"Phase I\"".to_owned(),
        city: "Bangaluru Karnataka".to_owned(),
    };

    let address_string = serde_json::to_string(&address)?;
    println!("Stringified address: {}", address_string);

    Ok(())
}
