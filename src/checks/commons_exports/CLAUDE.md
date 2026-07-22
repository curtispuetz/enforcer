<commons-exports-check>
    <desc>
        Checks the root `mod.rs` of every `t/` and `s/` commons directory. For a `t/` directory, every top-level public `struct`, `enum`,
        `trait`, and `type` alias defined anywhere in the directory must be re-exported by a
        `pub use` in the root `mod.rs`. For an `s/` directory, the same for every `static`
        defined anywhere. In both cases every module declared in the root `mod.rs` must be private (`mod x;`, never `pub mod x;`).
    </desc>
    <rationale>
        Users import flat: `use ...::t::TypeName`, never `use ...::t::submodule::TypeName`.
    </rationale>
    <note>
        A glob re-export (`pub use submodule::*;`) in the root `mod.rs` is treated as covering every
        public name, so missing-export checks are skipped for that directory.
    </note>
</commons-exports-check>
