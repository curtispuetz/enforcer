const MAX_WIDTH: usize = 40;

pub fn bar(count: usize, most: usize) -> String {
    "█".repeat(count * MAX_WIDTH / most.max(1))
}
