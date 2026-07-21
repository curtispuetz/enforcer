<type-location-check>
    <desc>
        Checks that every public `struct`, `enum`, `trait`, and `type` alias is defined in a `t/`
        directory. A public type defined anywhere outside a `t/` directory is a violation. Private
        (non-`pub`) types are allowed anywhere. Only top-level items are checked.
    </desc>
</type-location-check>
