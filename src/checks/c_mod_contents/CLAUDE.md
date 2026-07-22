<c-mod-contents-check>
    <desc>
        Checks the root `mod.rs` of every `c/` directory (a `mod.rs` whose parent folder is named
        `c`). By default such a file must declare all of its modules as public (`pub mod ...`) and
        contain no `use` statements.
    </desc>
    <exception>
        If the `c/` directory contains a nested `c/` sub-directory, the root `mod.rs` must instead:
        <item>have the inner attribute `#![allow(clippy::module_inception)]`</item>
        <item>declare the nested module privately as `mod c;`</item>
        <item>
            have a single `pub use c::{...}` statement re-exporting the nested module's public
            items, alongside its other `pub mod` statements.
        </item>
    </exception>
</c-mod-contents-check>
