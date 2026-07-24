use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use syn::Block;

// not-obvious: `DefaultHasher` has fixed keys, so its output is stable across
// runs (unlike `RandomState`), which is required for a copy-pasteable ignore id
pub fn digest(canonical: &Block) -> String {
    let mut hasher = DefaultHasher::new();
    canonical.hash(&mut hasher);
    format!("{:08x}", (hasher.finish() & 0xffff_ffff) as u32)
}
