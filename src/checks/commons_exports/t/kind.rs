#[derive(Clone, Copy)]
pub enum Kind {
    Types,
    Statics,
}

impl Kind {
    pub fn noun(self) -> &'static str {
        match self {
            Kind::Types => "type",
            Kind::Statics => "static",
        }
    }
}
