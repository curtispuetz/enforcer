use std::{process, sync::LazyLock};

use serde::Deserialize;

use crate::s::main::ROOT;

pub static FILE_CONFIG: LazyLock<FileConfig> = LazyLock::new(|| {
    let path = ROOT.join("rustenforcer.toml");
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return FileConfig::default();
    };
    toml::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("enforcer: invalid config in {}", path.display());
        eprintln!("{e}");
        process::exit(2);
    })
});

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FileConfig {
    #[serde(default)]
    pub import_rules: ImportRules,
    #[serde(default)]
    pub less_than_lines: LessThanLines,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ImportRules {
    #[serde(default)]
    pub ignore_export_macros: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LessThanLines {
    #[serde(default = "_default_num")]
    pub num: usize,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for LessThanLines {
    fn default() -> Self {
        LessThanLines {
            num: _default_num(),
            ignore: Vec::new(),
        }
    }
}

fn _default_num() -> usize {
    100
}
