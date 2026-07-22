use serde::Deserialize;

use super::{CognitiveComplexity, CommentRules, FileSizes, ImportRules, ModCount};

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FileConfig {
    #[serde(default)]
    pub import_rules: ImportRules,
    #[serde(default)]
    pub file_sizes: FileSizes,
    #[serde(default, rename = "comment-rules")]
    pub comment_rules: CommentRules,
    #[serde(default)]
    pub mod_count: ModCount,
    #[serde(default, rename = "cognitive-complexity")]
    pub cognitive_complexity: CognitiveComplexity,
}
