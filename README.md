# enforcer

A cargo subcommand that enforces structural rules on a Rust codebase, such as:

- How big files are
- The maximum allowed SonarQube cognitive complexity of a function
- No duplicated functions or duplicate runs of statements
- Limited comments

The main ideas is to use this enforcer with AI agents, telling the AI agent to run these after each change and keeping all the rules. 

## Install

```sh
cargo install enforcer
```

## Usage

Run from within the crate you want to check:

```sh
cargo enforcer all
```

Or run individual checks, one or several at a time:

```sh
cargo enforcer file-sizes cognitive-complexity
```

The process exits `1` when any check fails and `2` on a bad invocation

## Checks

| Check | Verifies |
| --- | --- |
| `file-sizes` | No file exceeds a maximum line count (99 default; configurable) |
| `cognitive-complexity` | Every function scores below a maximum SonarQube cognitive complexity (8 default; configurable) |
| `duplicate-fns` | No two functions are alpha-equivalent |
| `duplicate-logic` | No run of consecutive statements is repeated |
| `comment-rules` | No comments except short trailing ones and those prefixed `not-obvious: ` |
| `mod-count` | No `mod.rs`/`lib.rs` declares more than a maximum number of modules |
| `mod-location` | `mod` statements appear only in `mod.rs` and `lib.rs` |
| `mod-over-file` | Folder modules use the `mod.rs` form, not the sibling-file form |
| `mod-lib-contents` | `mod.rs`/`lib.rs` contain only `mod` and `use` statements |
| `use-privacy` | `use` statements outside `mod.rs`/`lib.rs` are private |
| `private-fn-naming` | Private functions and methods are prefixed with `_` |
| `call-rules` | Public functions are called through a parent module path, with no repeated words in the path |
| `tree-structure` | Imports point only sideways or deeper, never up or across; items live in their designated common module; impls sit beside their types; shared `c` functions are used by more than one branch |

`all` runs every check above.

### The module tree

The rules lean on five conventional module names for shared code, any of which
may be a folder or a single file:

- `t` — types (`struct`, `enum`, `trait`, `type` alias)
- `c` — free functions
- `s` — `static` items
- `cnst` — `const` items
- `ext_traits` — extension traits for types you don't own

A file may import its siblings and anything nested below it using `super::`, or a
common module at or above it using `crate::`. It may not reach up into a parent's
own logic or sideways into a different branch. The effect is a tree where
submodules only ever support their parents.

## Configuration

Checks read an optional `enforcer.toml` in the project root. Every key is
optional; omit the file entirely to take the defaults.

```toml
[file-sizes]
max = 99
ignore = ["src/generated/big_table.rs"]

[mod-count]
max = 10
ignore = ["src/checks"]

[cognitive-complexity]
max = 8
ignore = ["src/main.rs::run", "src/config.rs::Config::new"]

[duplicate-fns]
ignore = ["src/checks/call_rules/main.rs::run"]

[duplicate-logic]
min_stmts = 2
ignore = ["b841dd53"]

[comment-rules]
max_trailing_comment_len = 20

[tree-structure.import-rules]
ignore_export_macros = false
```

`duplicate-logic` group ids are printed with each violation; copy them into
`ignore` to silence a group.

## Opinions

These rules are opinionated, and deliberately so — they encode one particular
way of laying out a crate. Adopting `enforcer` wholesale on an existing codebase
will produce a lot of violations. Enabling a few checks at a time tends to work
better than starting with `all`.

## License

[MIT](LICENSE)
