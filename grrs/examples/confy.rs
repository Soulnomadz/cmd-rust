use serde::{Serialize, Deserialize};

// use std::io;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { name: "John".into(), comfy: false,  foo: 0 } }
}

fn main() -> Result<()> {
    let cfg: MyConfig = confy::load_path("e:/tmp/my_app/test.toml")?;
    println!("{:#?}", cfg);
    Ok(())
}
