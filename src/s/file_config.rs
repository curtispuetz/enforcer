use std::sync::LazyLock;

use serde::Deserialize;

use crate::s::main::ROOT;

pub static FILE_CONFIG: LazyLock<FileConfig> = LazyLock::new(|| {
    let path = ROOT.join("rustenforcer.toml");
    let Ok(contents) = std::fs::read_to_string(path) else {
        return FileConfig::default();
    };
    toml::from_str(&contents).unwrap_or_default()
});

#[derive(Deserialize, Default)]
pub struct FileConfig {
    #[serde(default)]
    pub import_rules: ImportRules,
}

#[derive(Deserialize, Default)]
pub struct ImportRules {
    #[serde(default)]
    pub ignore_export_macros: bool,
}
