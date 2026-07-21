<file-sizes-check>
    <desc>
        Checks that no source file is too long. A file fails if it has at or above a maximum
        number of lines.
    </desc>
    <config>
        Configurable in rustenforcer.toml under `[file_sizes]`:
        <item>num - max number of lines (default 100)</item>
        <item>ignore - list of exempt files from the rule</item>
    </config>
</file-sizes-check>
