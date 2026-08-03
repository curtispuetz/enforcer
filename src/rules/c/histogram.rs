use {super::files, colored::Colorize, std::path::Path};

const MAX_BAR: usize = 40;

pub fn measure(each: impl Fn(&Path) -> Vec<usize>) -> Vec<usize> {
    let mut values = Vec::new();
    for path in files::all() {
        values.extend(each(&path));
    }
    values
}

pub fn plot(values: &[usize], width: usize, max: usize, noun: &str, metric: &str) {
    if values.is_empty() {
        println!("{} nothing to measure", "[empty]".yellow().bold());
        return;
    }
    _bars(&_rows(values, width, max));
    println!("\n  {} {noun} measured", values.len());
    println!("  average {metric}: {:.2}", _average(values));
}

fn _rows(values: &[usize], width: usize, max: usize) -> Vec<(String, usize)> {
    let last = max / width;
    let counts = _counts(values, width, last);
    let first = counts.iter().position(|count| *count > 0).unwrap_or(0);
    counts
        .iter()
        .enumerate()
        .skip(first)
        .map(|(bin, count)| (_label(bin, width, last), *count))
        .collect()
}

fn _counts(values: &[usize], width: usize, last: usize) -> Vec<usize> {
    let mut counts = vec![0; last + 2];
    for value in values {
        counts[(value / width).min(last + 1)] += 1;
    }
    if counts[last + 1] == 0 {
        counts.pop();
    }
    counts
}

fn _label(bin: usize, width: usize, last: usize) -> String {
    let start = bin * width;
    if bin > last {
        return format!("{start}+");
    }
    if width == 1 {
        return start.to_string();
    }
    format!("{start}-{}", start + width - 1)
}

fn _bars(rows: &[(String, usize)]) {
    let most = rows.iter().map(|(_, count)| *count).max().unwrap_or(1);
    let pad = rows.iter().map(|(label, _)| label.len()).max().unwrap_or(1);
    for (label, count) in rows {
        println!(
            "  {}  {} {}",
            format!("{label:>pad$}").bold(),
            _bar(*count, most).cyan(),
            count.to_string().dimmed()
        );
    }
}

fn _average(values: &[usize]) -> f64 {
    let sum: usize = values.iter().sum();
    sum as f64 / values.len().max(1) as f64
}

fn _bar(count: usize, most: usize) -> String {
    "█".repeat(count * MAX_BAR / most.max(1))
}
