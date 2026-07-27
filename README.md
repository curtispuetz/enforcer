# enforcer

A cargo subcommand that enforces structural rules on a Rust codebase, such as:

- How big files are
- The maximum allowed SonarQube cognitive complexity of a function
- No duplicated functions or duplicate runs of statements
- Limited comments

The main idea is to use this enforcer with AI agents, telling the agent to run these after each change it makes so that it has to fix any rule violations. Starting a project from scratch with this will force the agents to not have large files, complex functions, and duplicated logic.

## Install

```sh
cargo install enforcer
```

## Usage

Run from within the crate you want to check:

```sh
cargo enforcer all
```

Or run individual rules, one or several at a time:

```sh
cargo enforcer file-sizes cognitive-complexity
```

The process exits `1` when any rule fails and `2` on a bad invocation

## Opinions

Some of the rules are opinionated, and deliberately so. They encode one particular way of laying out a crate. They might work well for a new project where you can run `enforcer` after each change, but adopting `enforcer` wholesale on an existing codebase will produce a lot of violations. Enabling just a set of the rules (like file-sizes, cognitive-complexity) would be more reasonable in that case.

## Rules

| Rule | Verifies |
| --- | --- |
| `file-sizes` | No file exceeds a maximum line count (99 default; configurable) |
| `cognitive-complexity` | Every function scores below a maximum SonarQube cognitive complexity (8 default; configurable) |
| `duplicate-fns` | No two functions are the same |
| `duplicate-logic` | No run of consecutive statements is repeated |
| `comments` | No comments except short trailing ones and those prefixed `not-obvious: ` |
| `mod-over-file` | Folder modules use the `mod.rs` form, not the sibling-file form |
| `mod-count` | No `mod.rs`/`lib.rs` declares more than a maximum number of modules |
| `mod-location` | `mod` statements appear only in `mod.rs` and `lib.rs` |
| `mod-lib-contents` | `mod.rs`/`lib.rs` contain only `mod` and `use` statements |
| `use-privacy` | `use` statements outside `mod.rs`/`lib.rs` are private |
| `private-fn-naming` | Private functions and methods are prefixed with `_` |
| `calls` | Public functions are called through a parent module path, with no repeated words in the path |
| `tree-structure` | Imports point only sideways or deeper, never up or across; items of particular kinds live in their designated common module, which can be imported from more freely |

`all` runs every rule above.

For complete descriptions of individual rules, along with configuration options:

```sh
cargo enforcer help file-size
```

## Configuration

Goes in an optional `enforcer.toml` in the project root.

Top-level:

- `debug` - bool

## Automated fixes

Add `--fix` to apply the automated fixes any of the selected rules offer, before the rules are checked:

```sh
cargo enforcer all --fix
```

Only `private-fn-naming` has a fix for now.

## License

[MIT](LICENSE)
