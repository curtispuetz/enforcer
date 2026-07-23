<commons-nesting-check>
    <desc>
        Checks that `t` and `s` commons modules (see import-rules check) are not nested inside another of the same kind. A `t` module that has a `t` ancestor (e.g. `t/t`, `t/foo/t`, or `t/foo/t.rs`) is a violation, and likewise for `s`.
    </desc>
</commons-nesting-check>
