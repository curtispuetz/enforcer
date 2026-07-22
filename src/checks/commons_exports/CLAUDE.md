<commons-exports-check>
    <desc>
        Checks the root `mod.rs` of every `t/` and `s/` commons directory. Every module declared
        in the root `mod.rs` must be private (`mod x;`, never `pub mod x;`) and must be glob
        re-exported by a `pub use x::*;` in that same `mod.rs`.
    </desc>
    <rationale>
        Users import flat: `use ...::t::TypeName`, never `use ...::t::submodule::TypeName`. Forcing
        the glob re-export guarantees every public item is reachable flat without the mod.rs having
        to name each one, so a newly added type can never be forgotten.
    </rationale>
</commons-exports-check>
