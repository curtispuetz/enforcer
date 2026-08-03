use {
    super::{classify, find, t::Counts},
    crate::rules::c::{files, path},
    colored::Colorize,
    std::{fs, path::Path},
};

const MAX_ALLOWED: usize = 2;

pub fn report() {
    let counts = _counts();
    println!("{}", "comments report:".bold().cyan());
    _print_heavy(&counts);
    _print_per_file(&counts);
    println!();
}

fn _counts() -> Vec<Counts> {
    let mut counts = Vec::new();
    for file_path in files::all() {
        counts.push(_file_counts(&file_path));
    }
    counts
}

fn _file_counts(file_path: &Path) -> Counts {
    let source = fs::read_to_string(file_path).unwrap_or_default();
    let mut counts = Counts {
        path: path::rel(file_path),
        not_obvious: 0,
        todo: 0,
    };
    for comment in find::comments(&source) {
        if classify::not_obvious(&comment) {
            counts.not_obvious += 1;
        } else if classify::todo(&comment) {
            counts.todo += 1;
        }
    }
    counts
}

fn _print_heavy(counts: &[Counts]) {
    println!(
        "\n{}\n",
        format!("files with more than {MAX_ALLOWED} not-obvious comments:").green()
    );
    let heavy: Vec<&Counts> = counts
        .iter()
        .filter(|file| file.not_obvious > MAX_ALLOWED)
        .collect();
    if heavy.is_empty() {
        println!("  none");
        return;
    }
    for file in _sorted(heavy) {
        println!(
            "  {}  {}",
            file.path.bold(),
            format!("{} not-obvious", file.not_obvious).red()
        );
    }
}

fn _sorted(mut heavy: Vec<&Counts>) -> Vec<&Counts> {
    heavy.sort_by(|a, b| b.not_obvious.cmp(&a.not_obvious).then(a.path.cmp(&b.path)));
    heavy
}

fn _print_per_file(counts: &[Counts]) {
    let files = counts.len();
    println!("\n  {files} files measured");
    _print_total("not-obvious", counts.iter().map(|f| f.not_obvious).sum(), files);
    _print_total("TODO", counts.iter().map(|f| f.todo).sum(), files);
}

fn _print_total(kind: &str, total: usize, files: usize) {
    let per_file = total as f64 / files.max(1) as f64;
    println!("  {kind} comments: {total} ({per_file:.3} per file)");
}
