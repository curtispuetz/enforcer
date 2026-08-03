use {
    super::{
        Calls, CognitiveComplexity, Comments, DuplicateFns, DuplicateLogic, FileSizes,
        ModCount, TreeStructure,
    },
    serde::Deserialize,
};

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct FileConfig {
    pub debug: bool,
    pub ignore: Vec<String>,
    pub tree_structure: TreeStructure,
    pub file_sizes: FileSizes,
    pub comments: Comments,
    pub mod_count: ModCount,
    pub cognitive_complexity: CognitiveComplexity,
    pub duplicate_fns: DuplicateFns,
    pub duplicate_logic: DuplicateLogic,
    pub calls: Calls,
}
