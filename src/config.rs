use std::{fs, path::Path};

use serde::{Serialize, de::DeserializeOwned};

pub trait Config
where
    Self: Serialize + DeserializeOwned,
{
    fn new(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    fn to_string_pretty(&self) -> Result<String, toml::ser::Error>
    where
        Self: Default,
    {
        toml::to_string_pretty(&Self::default())
    }
}

pub fn get_config<Conf, P>(path: P) -> Conf
where
    Conf: Config + Serialize + DeserializeOwned + Default,
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();
    if path.exists() {
        let data = fs::read_to_string(path).unwrap_or_else(|_| {
            panic!(
                "Error while parsing config file : {}",
                path.to_str().unwrap_or("No file path")
            )
        });
        toml::from_str(&data).unwrap()
    } else {
        save_default_conf(path)
    }
}

fn save_default_conf<Conf, P>(path: P) -> Conf
where
    Conf: Config + Serialize + DeserializeOwned + Default,
    P: AsRef<Path>,
{
    let default = Conf::default();
    fs::write(path, default.to_string_pretty().unwrap()).unwrap();
    default
}
