%{
  configs: [
    %{
      name: "default",
      files: %{
        included: ["lib/", "test/"],
        excluded: [~r"/_build/", ~r"/deps/"]
      },
      plugins: [],
      requires: [],
      strict: true,
      parse_timeout: 5000,
      color: true,
      checks: %{
        enabled: [
          {Credo.Check.Consistency.TabsOrSpaces, []},
          {Credo.Check.Readability.MaxLineLength, [max_length: 120]},
          {Credo.Check.Refactor.Nesting, [max_nesting: 3]},
          {Credo.Check.Refactor.CyclomaticComplexity, [max_complexity: 10]},
          {Credo.Check.Design.TagTODO, [exit_status: 0]},
          {Credo.Check.Design.TagFIXME, []}
        ]
      }
    }
  ]
}
