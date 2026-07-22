<mod-count-check>
    <desc>
        Limits how many modules live at each level. Checks that every `mod.rs` and `lib.rs` file declares at most a maximum number of modules (`mod` declarations).
    </desc>
    <config>
        Configurable in `enforcer.toml` under `[mod-count]`:
        <item>max - maximum `mod` declarations allowed per file (default 10).</item>
        <item>
            ignore - list of module paths to skip (the directory containing the `mod.rs`/`lib.rs` the check for, relative to the project root, e.g. `src/app`).
        </item>
    </config>
</mod-count-check>
