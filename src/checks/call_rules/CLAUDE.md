<call-rules-check>
    <desc>
        Checks how functions are called. Functions must never be imported directly and called
        bare; instead one of the function's parent modules is imported and the function is called
        through it as `parent::fn_name()` (or `upper::parent::fn_name()` for a higher parent).
        Additionally, across a qualified call's path segments no word may appear twice — the idea
        is never to see the same word twice. All the words in the segments are gathered and checked
        for duplicates, e.g. `app::get_app()` is a violation (prefer `app::get()`).
    </desc>
    <rules>
        <rule>
            Direct import: a bare call `fn_name()` whose name is brought into scope by a `use`
            statement is a violation — import a parent module and call it as `parent::fn_name()`.
            Only names that look like functions (starting lowercase or `_`) are considered, so
            tuple-struct/enum-variant constructors and type-associated calls are not flagged. Bare
            calls of locally-defined functions (not imported) are allowed.
        </rule>
        <rule>
            Repeated word: for a qualified call `a::b::fn_name()`, every path segment is split on
            `_` into words (compared case-insensitively) and the whole set must have no duplicate.
            The `crate`, `super`, `self`, and `Self` keyword segments are ignored when gathering
            words.
        </rule>
    </rules>
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
