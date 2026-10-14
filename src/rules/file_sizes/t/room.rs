pub enum Room {
    Uncapped,
    Free(usize),
    Full { count: usize, max: usize },
}
