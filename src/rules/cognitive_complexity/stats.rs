use {
    super::measure,
    crate::{
        rules::c::{files, histogram},
        s::EXISTING_SRC_DIRS,
    },
    colored::Colorize,
    std::collections::BTreeMap,
};

pub fn report() {
    let counts = _counts();
    println!("{}", "cognitive-complexity report:".bold().cyan());
    if counts.is_empty() {
        println!("{} no functions found", "[empty]".yellow().bold());
        return;
    }
    _print_rows(&counts);
    println!();
}

fn _counts() -> BTreeMap<usize, usize> {
    let mut counts = BTreeMap::new();
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            for function in measure::functions(&files::ast_parse(&file_path)) {
                *counts.entry(function.score).or_insert(0) += 1;
            }
        }
    }
    counts
}

fn _print_rows(counts: &BTreeMap<usize, usize>) {
    println!("\n{}\n", "functions per cognitive complexity score:".green());
    let min = counts.keys().next().copied().unwrap_or(0);
    let max = counts.keys().next_back().copied().unwrap_or(0);
    let most = counts.values().copied().max().unwrap_or(1);
    let total: usize = counts.values().sum();
    for score in min..=max {
        let count = counts.get(&score).copied().unwrap_or(0);
        println!(
            "  {}  {} {}",
            format!("{score:>3}").bold(),
            histogram::bar(count, most).cyan(),
            count.to_string().dimmed()
        );
    }
    println!("\n  {total} functions scored");
    println!("  average score: {:.2}", _average(counts));
}

fn _average(counts: &BTreeMap<usize, usize>) -> f64 {
    let total: usize = counts.values().sum();
    let sum: usize = counts.iter().map(|(score, count)| score * count).sum();
    sum as f64 / total.max(1) as f64
}
