# TODOs

## Checks

- Disallow std certain macros. (Just a preference, I think some of these match code less readable) The list right now:
  - matches
  - assert_matches

## Other

- Consider if its possible to incorporate shorted imports, for example, if I don't want such a long use statement, I can reexport modules in lib.rs
  - Fable says that it is possible, and told me about some algorithms. It could replace the existing duplicate-fns check because it does that also I think. It isn't bound just by functions repeated logic.
