use serde::Deserialize;

use super::{
    CognitiveComplexity, CommentRules, DuplicateFns, FileSizes, ImportRules, ModCount,
};

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FileConfig {
    #[serde(default)]
    pub import_rules: ImportRules,
    #[serde(default)]
    pub file_sizes: FileSizes,
    #[serde(default)]
    pub comment_rules: CommentRules,
    #[serde(default)]
    pub mod_count: ModCount,
    #[serde(default)]
    pub cognitive_complexity: CognitiveComplexity,
    #[serde(default)]
    pub duplicate_fns: DuplicateFns,
}
