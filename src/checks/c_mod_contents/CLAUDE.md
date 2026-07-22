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
    <reexport-completeness>
        The `pub use c::{...}` must re-export every public name of the nested `c` module. The
        nested module's public names are gathered from its own `c/mod.rs` (the `pub mod` idents
        plus the exposed names of its `pub use` re-exports); any that are missing from the outer
        `pub use c::{...}` group are reported. A `pub use c::*` glob satisfies this by re-exporting
        everything.
    </reexport-completeness>
</c-mod-contents-check>
