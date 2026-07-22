<call-rules-check>
    <desc>
        Checks how functions are called.
        <rule>
            If a functions is imported directly and called bare, its a violation; instead one of the function's parent modules is imported and the function is called through it as `parent::fn_name()` (or `...::parent::fn_name()` for higher parent imports).
        </rule>
        <rule>
            Across a qualified call's path segments no word may appear twice. All the words in the segments are gathered and checked for duplicates, e.g. `app::get_app()` is a violation (prefer `app::get()`).
        </rule>
        <note>
            These rules only apply for functions that we own (not for external crates).
        </note>
    </desc>
    <code-architecture>
        `main::run` scans every `.rs` file in each existing src dir; `_check_file` parses the file
        and calls `find::violations`, collecting a `Violation` per file with a list of offending
        calls. Inside `find/`: `imports.rs` maps each `use` binding name to its full path;
        `calls.rs` walks the syn AST (via `syn::visit`) collecting the path segments of every
        `Expr::Call` with a plain path callee; `words.rs` decides whether a name looks like a
        function and finds a duplicate word among segments; `message.rs` formats the two violation
        kinds; `main.rs` orchestrates. `report::report` prints the pass/fail summary.
    </code-architecture>
</call-rules-check>
