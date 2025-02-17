use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{errors::CliError, APP_NAME};

pub fn process_config_cmd(
    key: impl AsRef<str>,
    value: impl ToString,
) -> Result<(), CliError> {
    let path = get_config_path()?;

    let mut config = load_config(&path).unwrap_or_else(|e| {
        println!("got error: {}", e);
        Default::default()
    });
    let key = key.as_ref();
    let value = value.to_string();

    match key {
        "git.repository" => {
            config.repository = Some(
                url::Url::parse(&value).map_err(CliError::WrongUrlFormat)?,
            )
        }
        "git.token" => config.token = Some(value),
        _ => {
            println!("Unknown key  \"{}\"", key);
            return Ok(());
        }
    }

    let contents = toml::to_string_pretty(&config)
        .map_err(|e| CliError::SerializeConfig(path.to_path_buf(), e))?;

    std::fs::write(&path, contents)
        .map_err(|e| CliError::WriteFile(path.to_path_buf(), e))?;

    Ok(())
}

pub fn load_config(path: impl AsRef<Path>) -> Result<Config, CliError> {
    let path = path.as_ref();

    let contents = std::fs::read_to_string(&path)
        .map_err(|e| CliError::ReadFile(path.to_path_buf(), e))?;
    let config = toml::from_str::<Config>(&contents)
        .map_err(|e| CliError::Deserialize(path.to_path_buf(), e))?;

    Ok(config)
}

fn get_config_path() -> Result<PathBuf, CliError> {
    let config_dir = dirs::config_dir().ok_or(CliError::HomeDirNotFound)?;
    let app_config_dir = config_dir.join(APP_NAME); // Название приложения
    std::fs::create_dir_all(&app_config_dir).map_err(CliError::CreatingDirs)?;

    Ok(app_config_dir.join("config.toml"))
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub repository: Option<Url>,
    pub token: Option<String>,
}
