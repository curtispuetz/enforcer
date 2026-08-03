use {crate::rules::s::COMMON, std::path::Path};

pub fn common_file_kind(path: &Path) -> Option<&'static str> {
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
        return None;
    }
    let stem = path.file_stem()?.to_str()?;
    COMMON.into_iter().find(|c| *c == stem)
}
