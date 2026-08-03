use {
    super::{count, t::Config},
    crate::{
        rules::c::{files, histogram},
        s::EXISTING_SRC_DIRS,
    },
    colored::Colorize,
};

const BIN_WIDTH: usize = 10;

pub fn report() {
    let lines = _line_counts();
    println!("{}", "file-sizes report:".bold().cyan());
    if lines.is_empty() {
        println!("{} no files found", "[empty]".yellow().bold());
        return;
    }
    _print_rows(&lines, Config::new().max_lines);
    println!();
}

fn _line_counts() -> Vec<usize> {
    let mut lines = Vec::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            lines.push(count::lines(&file_path));
        }
    }
    lines
}

fn _print_rows(lines: &[usize], max_lines: usize) {
    println!("\n{}\n", "files per line count:".green());
    let counts = _bin_counts(lines, max_lines);
    let most = counts.iter().copied().max().unwrap_or(1);
    for (bin, count) in counts.iter().enumerate() {
        println!(
            "  {}  {} {}",
            format!("{:>9}", _bin_label(bin, max_lines)).bold(),
            histogram::bar(*count, most).cyan(),
            count.to_string().dimmed()
        );
    }
    println!("\n  {} files measured", lines.len());
    println!("  average lines: {:.2}", _average(lines));
}

fn _bin_counts(lines: &[usize], max_lines: usize) -> Vec<usize> {
    let last = max_lines / BIN_WIDTH;
    let mut counts = vec![0; last + 2];
    for count in lines {
        counts[(count / BIN_WIDTH).min(last + 1)] += 1;
    }
    if counts[last + 1] == 0 {
        counts.pop();
    }
    counts
}

fn _bin_label(bin: usize, max_lines: usize) -> String {
    if bin > max_lines / BIN_WIDTH {
        return format!("{}+", bin * BIN_WIDTH);
    }
    format!("{}-{}", bin * BIN_WIDTH, bin * BIN_WIDTH + BIN_WIDTH - 1)
}

fn _average(lines: &[usize]) -> f64 {
    let sum: usize = lines.iter().sum();
    sum as f64 / lines.len().max(1) as f64
}
