use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("home dir not found")]
    HomeDirNotFound,

    #[error("failed to create dirs: {0}")]
    CreatingDirs(std::io::Error),

    #[error("failed to read file {0}: {1}")]
    ReadFile(std::path::PathBuf, std::io::Error),

    #[error("failed to write file {0}: {1}")]
    WriteFile(std::path::PathBuf, std::io::Error),

    #[error("failed to deserialize file {0}: {1}")]
    Deserialize(std::path::PathBuf, toml::de::Error),

    #[error("failed to serialize config {0}: {1}")]
    SerializeConfig(std::path::PathBuf, toml::ser::Error),

    #[error("failed to parse url from string: {0}")]
    WrongUrlFormat(url::ParseError),

    #[error(
        "repository not set, please use pinbox config git.repository <url>"
    )]
    RepositoryNotSet,

    #[error("failed to clone repository: {0}")]
    CloneRepo(git2::Error),

    #[error("failed to open repository: {0}")]
    OpenRepo(git2::Error),
}
