use {crate::rules::tree_structure::c::modules, std::path::Path};

pub fn is_t_common(path: &Path) -> bool {
    modules::in_common(path, "t")
}
