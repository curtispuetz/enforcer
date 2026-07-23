# TODOs

## Checks

- Verify a public function defined in a c module is used in more than one module. Rationale: a function in a c directory is there because it's needed in multiple modules. If the function is only used in one module, it can be moved to that module or one of its submodules.
- Verify the traits defined in an ext_traits module are only implemented:
  - Inside the ext_traits
  - For types that are external (not owned by our code)
- Disallow std certain macros. (Just a preference, I think some of these match code less readable) The list right now:
  - matches
  - assert_matches

## Other
