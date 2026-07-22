# TODO checks

- For each mod.rs and lib.rs file, the maximum number of mod declarations should be 10. This number should also be configurable in the rustenforcer.toml file (so, 10 is the default). The idea is that this limits the number of modules at each level. Also, there is an 'ignore' config as well that allows you skip the check for certain modules (just like the ignore config for file-sizes check)
