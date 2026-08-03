use {
    super::{mod_lib_contents, mod_location, mod_over_file, use_privacy},
    crate::rules::c::parts,
};

pub fn run() -> bool {
    let reports = vec![
        mod_over_file::part(),
        mod_location::part(),
        mod_lib_contents::part(),
        use_privacy::part(),
    ];
    parts::print("module-files", reports)
}
