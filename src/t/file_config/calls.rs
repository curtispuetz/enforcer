use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Calls {
    #[serde(default)]
    pub ignore: Vec<String>,
}
