<claude-onboarding>
    <preamble>
        This file provides guidance to Claude Code (claude.ai/code) when working with code in this
        repository.
    </preamble>
    <what-this-is>
        This is a crate called 'enforcer' that does various checks on a Rust codebase. Each check is
        a set of rules. The enforcer runs that check telling you if your codebase has any
        violations.
    </what-this-is>
    <architecture>
        <entrypoints>
            The crate builds a `cargo-enforcer` binary (see Cargo.toml `[[bin]]`). `src/main.rs`
            is the thin CLI shell: it reads the subcommand argument (skipping the leading
            `enforcer` arg that cargo injects for `cargo enforcer &lt;check&gt;`), parses it into a
            `Command`, and calls `command.run()`.
        </entrypoints>
        <commands-dispatch>
            `src/t/command.rs` defines the `Command` enum. `Command::run` dispatches to
            each check's module (e.g. `import_rules::run()`). Adding a check = new enum variant +
            new check module.
        </commands-dispatch>
        <shared-code>
            `src/s/` holds static data. `src/c/` is shared logic. `src/t/` is shared types.
        </shared-code>
        <check-impls>
            Asside from the shared-code, each check is implemented in its own folder under `src/checks`. In each folder, there is a description of the check in CLAUDE.md.
        </check-impls>
        <enforcer.toml>
            `src/s/file_config.rs` exposes the `FILE_CONFIG` static: a `LazyLock` that reads
            `enforcer.toml` from the project root (`ROOT`), allows users to configure the checks.
        </enforcer.toml>
        <std-out>
            All checks should print good messages to the console about the results using nice color schemes following the pattern already laid out. A message of the "all passed" sort if all pass, and if failures should print details, so the user knowns where the failures happened and can click links, if possible, to go fix. The number of passes and failures should be printed in all cases, like it is for the existing checks.
        </std-out>
    </architecture>
    <cargo-commands>
        <build>cargo build</build>
        <test>
            There is no test suite; verification is by building and manual testing.
        </test>
        <clippy>cargo clippy</clippy>
        <import-rules>
            cargo enforcer import-rules.
        </import-rules>
        <notes>
            clippy and import-rules should be run after each change to make sure there are no
            warnings. For the import-rules check, the idea is to put all static data in 's' dirs and
            public types in 't' dirs. There can also be some basic impl's for simple methods in the
            't'
            directories alongside the types. 'ext_traits' are for extension traits (adding traits to
            types
            we don't own). Principle: minimize c (ideally nothing in it), and place shared code as
            deep as
            possible, right where it's used. Principle: put all types used multiple places in 't'
            directories.
        </notes>
    </cargo-commands>
    <code-preferences>
        <file-length>
            Try to keep files short. Under 100 lines is a good rule-of-thumb, but regularly files
            are under 50 lines. Therefore, spread implementation
            across different logical units in different files, and group the logical units into
            folders.
        </file-length>
        <comments>
            Try to keep functions/methods short, and don't add comments to code about what something
            does. Instead, if you want to add a comment about what something does, you should create
            a
            function/method with a descriptive name, because this way, the function/method name acts
            as a
            comment, which means the code comments itself. However, comments about something obscure
            that's not very obvious from the code are fine.
        </comments>
    </code-preferences>
    <rust-code-preferences>
        <use-at-top>
            Never add a 'use' statement inside a function. Instead, always put use statements at
            the top of
            the file.
        </use-at-top>
        <module-fns>
            If it works nicely, instead of having a function named like
            animate_heal_character(), you can have a function named character in a module heal which
            is in
            a module animate, so them you import animate and do animate::heal::character() instead.
        </module-fns>
        <private-fns>
            Whenever defining private functions/methods (i.e. without 'pub'), start the
            function/method name with an
            underscore.
        </private-fns>
        <raw-loops>
            Prefer raw for loops to initialize collections rather than using
            iter().map(...).filter(...).collect() (and that), unless in some cases where it's a
            small
            iter()
            with a simple filter(...), or it's being used inline.
        </raw-loops>
        <mod-lib>
            In mod.rs and lib.rs files, only have 'mod' and 'use' statements (no other code)
        </mod-lib>
        <pub-super>Don't use pub</pub-super>
        <clippy>Follow clippy recommendations</clippy>
        <import-rules>
            <general>
                A file's dependencies may point sideways (to siblings in its own folder) or
                deeper (into nested modules), but never up to a parent folder or across into a
                different
                branch of the tree. The default: a file may import from its own module folder and
                anything
                nested inside it. `crate::app::tabs` can use a sibling `crate::app::editor` and a
                child
                `crate::app::tabs::helpers`, but not its parent's neighbor `crate::project`. This is
                what
                keeps a leaf from reaching across the codebase.
            </general>
            <use-super>
                Sideways and deeper imports must be written with `super::`, never
                `crate::`. A single `super` points at the containing folder, so `super::sibling`
                and `super::child::thing` stay within the file's own subtree by construction —
                the form itself is the proof of direction. Never use `super::super::...`: that
                walks up out of the folder, which is exactly the "reaching up" this rule forbids.
                Reserve `crate::` for the one case `super` cannot express: reaching a commons
                folder that lives above your own folder (see &lt;commons-folders&gt;).
            </use-super>
            <commons-folders>
                c/, t/, s/, ext_traits/ — shared code, scoped to one level. All four behave the
                same way. A commons folder is reachable by everything at or below its parent:
                `crate::c` is the whole-crate commons (any file may import it); `crate::app::c` is
                app-local commons (only files under `crate::app` may import it). Inside a commons
                folder the default rule starts over, as if that folder were the crate root — so a
                commons folder nested in another scopes its own commons the same way.
                <single-file>
                    When a commons' contents are small, it may be a single-file module (`t.rs`)
                    instead of a folder (`t/mod.rs`); both resolve to the same module path and
                    behave identically for import purposes. Use the file form to avoid a folder
                    holding one tiny type or static.
                </single-file>
                <note>
                    So a shared item goes in the commons for its kind — `c` for logic, `t` for
                    types, `s` for static data, `ext_traits` for extension traits — at the
                    narrowest level that covers every file needing it.
                </note>
            </commons-folders>
            <general-guide>
                The idea is to put all static data in 's' dirs and public types in 't' dirs. There
                can also
                be some basic impl's for simple methods in the 't' directories alongside the types.
                'ext_traits' are for extension traits (adding traits to types we don't own).
                Principle:
                minimize c (ideally nothing in it), and place shared code as deep as possible, right
                where
                it's used. Principle: put all types used multiple places in 't' directories. Run the
                `cargo
                enforcer import-rules` command to verify that imports follow these rules.
            </general-guide>
        </import-rules>
    </rust-code-preferences>
</claude-onboarding>
