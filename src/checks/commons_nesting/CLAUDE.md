<commons-nesting-check>
    <desc>
        Checks that `t/` and `s/` commons directories (see import-rules check) are not nested inside another directory of the
        same kind. A `t/` directory that has a `t/` ancestor (e.g. `t/t` or `t/foo/t`) is a
        violation, and likewise for `s/`.
    </desc>
</commons-nesting-check>
