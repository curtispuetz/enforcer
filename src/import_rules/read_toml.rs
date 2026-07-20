use serde::Deserialize;

use crate::s::main::ROOT;

pub fn ignore_export_macros() -> bool {
    _read().import_rules.ignore_export_macros
}

fn _read() -> FileConfig {
    let path = ROOT.join("rustenforcer.toml");
    let Ok(contents) = std::fs::read_to_string(path) else {
        return FileConfig::default();
    };
    toml::from_str(&contents).unwrap_or_default()
}

#[derive(Deserialize, Default)]
struct FileConfig {
    #[serde(default)]
    import_rules: ImportRules,
}

#[derive(Deserialize, Default)]
struct ImportRules {
    #[serde(default)]
    ignore_export_macros: bool,
}
