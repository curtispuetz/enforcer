<cognitive-complexity-check>
    <desc>
        Measures the cognitive complexity of every function (free functions, inherent and trait impl
        methods, and trait default methods) following SonarQube's cognitive complexity rules, and
        flags any function whose score exceeds a maximum. Cognitive complexity rewards linear code
        and penalizes deep nesting, so it tracks how hard code is for a human to follow rather than
        the number of independent paths (cyclomatic complexity).
    </desc>
    <config>
        Configurable in `enforcer.toml` under `[cognitive-complexity]`:
        <item>max - maximum cognitive complexity allowed per function (default 15).</item>
        <item>
            ignore - list of file paths to skip, relative to the project root (e.g. `src/main.rs`).
        </item>
    </config>
    <scoring>
        <structural>
            +1 (plus the current nesting depth) for each `if`, `match`, `for`, `while`, and `loop`.
            The nesting depth increases inside their bodies, so the same construct costs more the
            deeper it sits.
        </structural>
        <hybrid>
            +1 (with no nesting penalty) for each `else if` and `else`.
        </hybrid>
        <nesting-only>
            Closures and nested functions add no increment of their own but do increase the nesting
            depth for the code inside them.
        </nesting-only>
        <flat>
            +1 for each labeled `break`/`continue`, and +1 for each sequence of like boolean
            operators (`&&`/`||`), where a change of operator starts a new sequence.
        </flat>
        <not-counted>
            The `?` operator, unlabeled `break`/`continue`, and recursion are not counted.
        </not-counted>
    </scoring>
    <architecture>
        The scorer lives in `measure/score.rs` and walks each function body with a `syn::visit::Visit` implementation, carrying a `nesting` depth.
    </architecture>
</cognitive-complexity-check>
