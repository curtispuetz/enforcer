<commons-exports-check>
    <desc>
        Checks the root `mod.rs` of every `t/` and `s/` commons directory. Every module declared in the root `mod.rs` must be private (i.e. `mod x;`, never `pub mod x;`) and must be glob re-exported by a `pub use x::*;` in that same `mod.rs`.
    </desc>
    <rationale>
        Users import flat: `use ...::t::TypeName`, never `use ...::t::submodule::TypeName`.
    </rationale>
</commons-exports-check>
