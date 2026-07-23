pub fn misplaced_impl(name: &str) -> String {
    format!("impl {name} is not in the same module as the public {name} type definition")
}

pub fn foreign_trait_impl(name: &str) -> String {
    format!("impl of a trait for foreign type {name} must live in an ext_traits module")
}

pub fn free_fn(segments: &[String]) -> String {
    let call = segments.join("::");
    format!("{call}() reaches out to a crate free function; t modules must stay self-contained")
}
