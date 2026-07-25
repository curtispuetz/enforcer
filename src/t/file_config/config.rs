use serde::Deserialize;

use super::{
    CognitiveComplexity, Comments, DuplicateFns, DuplicateLogic, FileSizes, ModCount,
    TreeStructure,
};

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FileConfig {
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub tree_structure: TreeStructure,
    #[serde(default)]
    pub file_sizes: FileSizes,
    #[serde(default)]
    pub comments: Comments,
    #[serde(default)]
    pub mod_count: ModCount,
    #[serde(default)]
    pub cognitive_complexity: CognitiveComplexity,
    #[serde(default)]
    pub duplicate_fns: DuplicateFns,
    #[serde(default)]
    pub duplicate_logic: DuplicateLogic,
}
