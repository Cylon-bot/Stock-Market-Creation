use serde::de::DeserializeOwned;
use serde_yaml::{self};
use thiserror::Error;

#[derive(Debug)]
pub struct YamlFile<T> {
    pub file_content: T,
}
#[derive(Debug, Error)]
pub enum YamlError {
    #[error(transparent)]
    YamlReadError(#[from] serde_yaml::Error),
    #[error(transparent)]
    FileOpenError(#[from] std::io::Error),
}

impl<T> YamlFile<T> {
    pub fn try_new(path: String) -> Result<Self, YamlError>
    where
        T: DeserializeOwned,
    {
        let file = std::fs::File::open(path)?;
        let file_content: T = serde_yaml::from_reader(file)?;
        Ok(YamlFile { file_content })
    }
}
