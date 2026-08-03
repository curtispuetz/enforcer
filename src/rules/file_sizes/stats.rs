use {
    super::{count, t::Config},
    crate::rules::c::histogram,
    colored::Colorize,
};

const BIN_WIDTH: usize = 10;

pub fn report() {
    let lines = histogram::measure(|path| vec![count::lines(path)]);
    println!("{}", "file-sizes report:".bold().cyan());
    println!("\n{}\n", "files per line count:".green());
    histogram::plot(&lines, BIN_WIDTH, Config::new().max_lines, "files", "lines");
    println!();
}
