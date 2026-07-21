<import-rules-check>
    <code-architecture>
        Flow:
        `main::run` builds a `Config` (from `t/config.rs`; reads `ignore_export_macros` from the
        `[import_rules]` section of `rustenforcer.toml` in the project root via `read_toml.rs`,
        defaulting to false if the file or key is absent, and,
        if set, collects `#[macro_export]` names via `macros.rs`), iterates every `.rs` file in
        each existing src dir, and calls `check::file::run` on each, then `report::report`
        prints the pass/fail summary and exits non-zero on any failure. Inside `check/`:
        `file.rs` orchestrates one file's check; `location.rs` derives a file's module-path
        segments and flags the exempt `s`/`t`/`ext_traits` dirs; `imports.rs` walks the syn AST
        to extract `crate::`-rooted use paths (expanding groups/globs/renames); `rules.rs`
        holds the core allow/deny logic including the `c`-directory recursion. This mirrors the
        &lt;import-rules&gt; preferences documented below — the check enforces the same rules the
        codebase itself follows.
    </code-architecture>
</import-rules-check>
