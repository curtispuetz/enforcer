pub fn underscores(source: &str, spots: &[(usize, usize)]) -> String {
    let mut lines: Vec<Vec<char>> =
        source.split('\n').map(|l| l.chars().collect()).collect();
    let mut sorted = spots.to_vec();
    sorted.sort_unstable();
    for (line, col) in sorted.into_iter().rev() {
        lines[line - 1].insert(col, '_');
    }
    lines
        .into_iter()
        .map(String::from_iter)
        .collect::<Vec<String>>()
        .join("\n")
}
