pub struct Violation {
    pub path: String,
    pub imports: Vec<BadImport>,
}

pub struct BadImport {
    pub text: String,
    pub reason: &'static str,
}
