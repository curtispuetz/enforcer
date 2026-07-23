<import-rules-check>
    <desc>
        Checks that a file's imports only point sideways (to siblings in its own module) or deeper (into nested modules), never up to a parent module or across into a different branch of the tree. Imports that reach up or across are violations.
        <rules>
            <rule>
                Sideways/deeper imports must be written with `super::`, never `crate::`. Because a single `super` resolves to the containing module and appending segments only goes deeper. So, a `super::`-rooted path can only ever point sideways or deeper. `use super::super::...` is a violation because it walks up out of the folder.
            </rule>
            <rule>
                `crate::`-rooted imports are allowed only when reaching a commons dir that lives above the file's own folder (see &lt;commons-dirs&gt; below); those genuinely go up, which `super` cannot express without `super::super`. A `crate::` path that reaches a non-commons target up/across is a violation.
            </rule>
        </rules>
    </desc>
    <commons-modules>
        c, t, s, ext_traits — shared-code (general shared logic, types, static data, and extension traits, respectively). Each may be a folder (`t/`) with a mod.rss or, a single-file module (`t.rs`). All four behave the same way: a commons module is reachable by everything at or below its parent (e.g. `crate::c` is the whole-crate commons shared logic, reachable everywhere; `crate::app::c` is app-local, reachable only from files at or below `crate::app`). Additionally, inside a commons folder, the import-rules apply as if that folder were the crate root.
        <rationale>
            A shared item lives in a commons module at some level above everything using it.
        </rationale>
    </commons-modules>
    <config>
        Configurable in `enforcer.toml` under `[import_rules]`:
        <item>
            ignore_export_macros - when true, imports of `#[macro_export]` macros are exempt (default false)
        </item>
    </config>
    <code-architecture>
        Flow:
        `main::run` builds a `Config` (from `t/config.rs`; reads `ignore_export_macros` from the
        `[import_rules]` section of `enforcer.toml` in the project root via `read_toml.rs`,
        defaulting to false if the file or key is absent, and,
        if set, collects `#[macro_export]` names via `macros.rs`), iterates every `.rs` file in
        each existing src dir, and calls `check::file::run` on each, then `report::report`
        prints the pass/fail summary and exits non-zero on any failure. Inside `check/`:
        `file.rs` orchestrates one file's check; `location.rs` derives a file's containing-folder
        segments (`file_dir_segments`) and its own module path (`own_module_segments`, which
        appends the file stem for a normal file but not for `mod.rs`/`lib.rs`) and flags the
        commons `c`/`s`/`t`/`ext_traits` dirs; `imports.rs` walks the syn AST to extract
        `crate::`/`super::`-rooted use paths (expanding groups/globs/renames); `rules.rs`
        resolves each path to an absolute module path (using `own_module` for `super`) and
        returns the violation reason, if any, including the commons-directory recursion. This
        mirrors the
        &lt;import-rules&gt; preferences documented below — the check enforces the same rules the
        codebase itself follows.
    </code-architecture>
</import-rules-check>
