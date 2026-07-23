use syn::ItemFn;

// A free (module-level) function, kept alongside where it was found so we can
// point the user at it if it turns out to be a duplicate.
pub struct FreeFn {
    pub path: String,
    pub name: String,
    pub line: usize,
    pub item: ItemFn,
}

// One member of a duplicate group (the AST is no longer needed once grouped).
pub struct Duplicate {
    pub path: String,
    pub name: String,
    pub line: usize,
}

// A set of two-or-more free functions that are alpha-equivalent to each other.
pub struct Group {
    pub members: Vec<Duplicate>,
}
