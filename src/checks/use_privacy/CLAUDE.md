<use-privacy-check>
    <desc>
        Checks that every `use` statement in a non-`mod.rs`/`lib.rs` file is private. A `pub use` (re-export) is only allowed in `mod.rs` and `lib.rs` files.
    </desc>
    <rationale>
        Want re-exports in the module-declaring files (`mod.rs`/`lib.rs`), which define each folder's public surface.
    </rationale>
</use-privacy-check>
