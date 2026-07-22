<c-mod-contents-check>
    <desc>
        Checks the root `mod.rs` of every `c/` directory (a `mod.rs` whose parent folder is named
        `c`) declares all of its modules as public (`pub mod ...`) and contains no `use` statements.
    </desc>
    <exception>
        If the `c/` directory contains a nested `c/` sub-directory, the root `mod.rs` must instead:
        <item>
            declare the nested module privately as `mod c;`, annotated with the outer attribute
            `#[allow(clippy::module_inception)]`
        </item>
        <item>
            have a single `pub use c::*;` glob statement re-exporting the nested `c` module.
        </item>
    </exception>
</c-mod-contents-check>
