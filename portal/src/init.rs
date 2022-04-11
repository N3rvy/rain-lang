use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use crate::Args;
use crate::config::Config;

pub fn init(args: Args) -> anyhow::Result<()> {
    let config = Config::default();

    let config_str = serde_json::to_string_pretty(&config)?;

    fs::create_dir_all(config.definition_dir.clone())?;
    fs::create_dir_all(config.src_dir.clone())?;

    let src_dir = config.src_dir.clone();
    let main_path = PathBuf::from_str(src_dir.as_str())?.join(config.main);
    let mut main_file = File::create(main_path.to_str().unwrap())?;
    main_file.write_all(br#"
    func main() none:
        return
    "#)?;

    let mut config_file = File::create(args.module)?;
    config_file.write_all(config_str.as_bytes())?;

    Ok(())
}