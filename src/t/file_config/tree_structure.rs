use {super::Imports, serde::Deserialize};

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct TreeStructure {
    pub imports: Imports,
}
