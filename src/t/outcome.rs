pub enum Outcome<V> {
    Skipped,
    Passed,
    Failed(V),
}
