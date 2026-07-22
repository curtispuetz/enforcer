<comment-rules-check>
    <desc>Checks that comments are justified.</desc>
    <allowed-conditions>
      A comment is allowed only if it either:
      <condition>
        starts with `not-obvious: ` — idea is to use this for something genuinely obscure that the code cannot make clear on its own
      </condition>
      <condition>Is a short trailing comment, up to a maximum length.</condition>
    </allowed-conditions>
    <config>
      Configurable in `rustenforcer.toml` under `[comment_rules]`:
      <item>max_trailing_comment_len (default 20)<item>
    </config>
</comment-rules-check>
