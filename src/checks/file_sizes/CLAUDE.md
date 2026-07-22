<file-sizes-check>
    <desc>
        Checks that no source file is too long. A file fails if it has more than a maximum
        number of lines.
    </desc>
    <config>
        Configurable in enforcer.toml under `[file_sizes]`:
        <item>max - max number of lines (default 99)</item>
        <item>ignore - list of exempt files from the rule</item>
    </config>
</file-sizes-check>
