use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    config::{get_config_path, load_config},
    errors::CliError,
    APP_NAME,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Categories {
    categories: Vec<Category>,
}

impl Default for Categories {
    fn default() -> Self {
        Categories {
            categories: vec![
                Category {
                    name: "video".to_string(),
                    alias: Some("Video".to_string()),
                },
                Category {
                    name: "article".to_string(),
                    alias: Some("Articles".to_string()),
                },
                Category {
                    name: "books".to_string(),
                    alias: Some("Books".to_string()),
                },
            ],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    name: String,
    alias: Option<String>,
}

pub fn get_categories(
    repo: git2::Repository,
    path: impl AsRef<Path>,
) -> Result<Categories, CliError> {
    let path = path.as_ref().join("config.toml");

    if !path.exists() {
        let config_path = get_config_path()?;
        let config = load_config(config_path)?;

        let contents = toml::to_string_pretty(&Categories::default())
            .map_err(|e| CliError::SerializeConfig(path.to_path_buf(), e))?;

        std::fs::write(&path, contents)
            .map_err(|e| CliError::WriteFile(path.to_path_buf(), e))?;

        let mut remote = repo.find_remote("origin").unwrap();

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |_url, _username, allowed_types| {
            let token = config.token.clone().unwrap();

            if allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT)
            {
                return git2::Cred::userpass_plaintext(
                    _username.unwrap_or("git"),
                    &token,
                );
            }
            Err(git2::Error::from_str("can not auth"))
        });

        let refspec = format!("refs/heads/{}", "main");
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(callbacks);

        remote
            .push(&[refspec.as_str()], Some(&mut push_options))
            .unwrap();
    }

    let contents = std::fs::read_to_string(&path)
        .map_err(|e| CliError::ReadFile(path.to_path_buf(), e))?;

    let categories = toml::from_str::<Categories>(&contents)
        .map_err(|e| CliError::Deserialize(path.to_path_buf(), e))?;

    Ok(categories)
}

pub fn get_repository(
) -> Result<(git2::Repository, std::path::PathBuf), CliError> {
    let path = get_repo_path()?;

    let repo =
        if path.exists() && std::fs::read_dir(&path).unwrap().count() == 0 {
            let config_path = get_config_path()?;
            let config = load_config(config_path)?;

            let repo = if let Some(url) = config.repository {
                git2::Repository::clone(url.as_str(), &path)
                    .map_err(CliError::CloneRepo)?
            } else {
                return Err(CliError::RepositoryNotSet);
            };

            repo
        } else {
            git2::Repository::open(&path).map_err(CliError::OpenRepo)?
        };

    Ok((repo, path))
}

pub fn get_repo_path() -> Result<PathBuf, CliError> {
    let repo_dir = dirs::config_dir().ok_or(CliError::HomeDirNotFound)?;
    let repo_dir = repo_dir.join(APP_NAME).join("pinbox-notes");
    std::fs::create_dir_all(&repo_dir).map_err(CliError::CreatingDirs)?;

    Ok(repo_dir)
}
