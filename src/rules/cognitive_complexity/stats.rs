use {
    super::{measure, t::Config},
    crate::rules::c::{files, histogram},
    colored::Colorize,
    std::path::Path,
};

pub fn report() {
    let scores = histogram::measure(_scores);
    println!("{}", "cognitive-complexity report:".bold().cyan());
    println!(
        "\n{}\n",
        "functions per cognitive complexity score:".green()
    );
    histogram::plot(&scores, 1, Config::new().max, "functions", "score");
    println!();
}

fn _scores(file_path: &Path) -> Vec<usize> {
    measure::functions(&files::ast_parse(file_path))
        .into_iter()
        .map(|function| function.score)
        .collect()
}
