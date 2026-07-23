# TODOs

## Checks

- Disallow a function defined in a c directory to be used only in one module. Rationale: a function in a c directory is there because it's needed in multiple modules. If the function is only used in one module, it can be moved to that module or deeper.
- Disallow std certain macros. (Just a preference, I think some of these match code less readable) The list right now:
  - matches
  - assert_matches

## Other
