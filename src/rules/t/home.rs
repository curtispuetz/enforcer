pub enum Home {
    File(String),
    Siblings {
        folder: String,
        files: Vec<String>,
    },
    Common {
        lowest: Vec<String>,
        common: Vec<String>,
        direct: bool,
        kept: Option<String>,
    },
}
