use serde_derive::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    version: u8,
    api_key: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: 0, api_key: "".into() } }
}

fn main() -> Result<()> {
    let mut cfg: MyConfig = confy::load("my-app-name", "test.toml")?;
    println!("{:?}", confy::get_configuration_file_path("my-app-name", "test.toml"));
    cfg.version = 1;
    confy::store("my-app-name", "test.toml", &cfg)?;
    println!("{:?}", cfg);

    Ok(())
}