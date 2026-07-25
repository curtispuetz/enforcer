# enforcer

A cargo subcommand that enforces structural rules on a Rust codebase, such as:

- How big files are
- The maximum allowed SonarQube cognitive complexity of a function
- No duplicated functions or duplicate runs of statements
- Limited comments

The main idea is to use this enforcer with AI agents, telling the AI agent to run these after each change and keeping all the rules.

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

For complete descriptions of individuals check, along with configuration options:

```sh
cargo enforcer help file-size
```

## Configuration

Goes in an optional `enforcer.toml` in the project root.

## Opinions

These rules are opinionated, and deliberately so. They encode one particular way of laying out a crate. They might work well for a new project where you can run it after each change, but adopting `enforcer` wholesale on an existing codebase will produce a lot of violations. Enabling just a set of the checks (like file-sizes, cognitive-complexity) might be more reasonable.

## License

[MIT](LICENSE)
