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
    pub file_sizes: FileSizes,
    #[serde(default, rename = "comment-rules")]
    pub comment_rules: CommentRules,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ImportRules {
    #[serde(default)]
    pub ignore_export_macros: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileSizes {
    #[serde(default = "_default_num")]
    pub num: usize,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for FileSizes {
    fn default() -> Self {
        FileSizes {
            num: _default_num(),
            ignore: Vec::new(),
        }
    }
}

fn _default_num() -> usize {
    100
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommentRules {
    #[serde(default = "_default_trailing_max")]
    pub max_trailing_comment_len: usize,
}

impl Default for CommentRules {
    fn default() -> Self {
        CommentRules {
            max_trailing_comment_len: _default_trailing_max(),
        }
    }
}

fn _default_trailing_max() -> usize {
    10
}
