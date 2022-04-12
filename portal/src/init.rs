use anyhow::anyhow;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::Args;
use crate::config::Config;

pub fn init(args: Args) -> anyhow::Result<()> {
    if Path::new(&args.module).exists() {
        return Err(anyhow!("Config file ({}) already exists in this directory", &args.module));
    }

    let config = Config::default();

    let config_str = serde_json::to_string_pretty(&config)?;

    fs::create_dir_all(config.definition_dir.clone())?;
    fs::create_dir_all(config.src_dir.clone())?;

    let main_path = Path::new(&config.src_dir).join(Path::new(&config.main));

    if main_path.exists() {
        return Err(anyhow!("Main file already exists in {}", main_path.to_str().unwrap()))
    }

    let mut main_file = File::create(main_path)?;
    main_file.write_all(br#"
func main() none:
    return
    "#)?;

    let mut config_file = File::create(args.module)?;
    config_file.write_all(config_str.as_bytes())?;

    Ok(())
}