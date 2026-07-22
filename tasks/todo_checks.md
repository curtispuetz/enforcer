# TODO checks

- For t/ and s/ directories, all public types and statics defined in the directories respectively are exported in the root mod.rs file, and all modules defined in the root mod.rs file are private. The idea is that users import as 'use ...t::TypeName'.
