# TODO checks

- [x] All public structs, enums, traits, and type definitions (defined with 'type' keyword) are in a t/ directory (private ones are fine wherever)
- [x] All public statics are in a s/ directory (private ones are fine wherever)
- Functions are never imported directly. Instead, at least one of the functions parent modules is imported, and the function is called as parent::fn_name().
