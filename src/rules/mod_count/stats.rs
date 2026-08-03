use {
    super::{count, t::Config},
    crate::rules::c::{histogram, path},
    colored::Colorize,
    std::path::Path,
};

pub fn report() {
    let mods = histogram::measure(_declared);
    println!("{}", "mod-count report:".bold().cyan());
    println!("\n{}\n", "mod.rs/lib.rs files per module count:".green());
    histogram::plot(&mods, 1, Config::new().max, "files", "modules");
    println!();
}

fn _declared(file_path: &Path) -> Vec<usize> {
    if path::is_mod_or_lib(file_path) {
        return vec![count::mods(file_path)];
    }
    Vec::new()
}
