<import-rules-check>
    <desc>
        Checks that a file's imports only point sideways (to siblings in its own folder) or
        deeper (into nested modules), never up to a parent folder or across into a different
        branch of the tree. Imports that reach up or across are violations.
    </desc>
    <exemptions>
        <item>
            c/ — shared code scoped to one level: a `c` folder is reachable by everything at or
            below its parent (e.g. `crate::c` would be the whole-crate commons; `crate::app::c` would be
            app-local). Inside a `c` folder the default rule starts over as if it were the crate
            root.
        </item>
        <item>
            t/, s/, ext_traits/ — universal leaves for types, static data, and extension traits.
            Any file may import them from anywhere, and a path is exempt if any of its segments
            is one of these.
        </item>
    </exemptions>
    <config>
        Configurable in `rustenforcer.toml` under `[import_rules]`:
        <item>
            ignore_export_macros - when true, imports of `#[macro_export]` macros are exempt
            (default false)
        </item>
    </config>
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
