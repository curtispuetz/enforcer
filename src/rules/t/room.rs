pub enum Room {
    Uncapped,
    Free(usize),
    Full,
}
