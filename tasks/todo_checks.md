# TODO checks

- Functions are never imported directly. Instead, at least one of the functions parent modules is imported, and the function is called as parent::fn_name().
- For t/ and s/ directories, all public types and statics defined are exported in the root mod.rs file. All modules defined in the root mod.rs file are private.
- For c/ directories, the root mod.rs file declares all its modules as public. The exception is if there is a sub c/ directory in the c directory, and in this case the root mod.rs must have a 'pub use c::{all sub c public things..}' statement.