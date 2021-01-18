use std::{
    io::prelude::*,
    fs,
};
use anyhow::Result;
use serde::{Serialize, Deserialize};

pub fn update(file: &str) -> Result<()> {
    //TODO fill Cache struct from network and then serialize to json
    fs::write(file, json)?;
    Ok(())
}

pub fn show() {
    todo!();
}

#[derive(Serialize, Deserialize, Debug)]
struct Cache {
}
