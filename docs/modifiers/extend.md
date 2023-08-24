Modification to extend the rule with subrules. This is a very important modifier, because it allows you to create a tree of rules, and also allows you to create a tree of rules inside a tree of rules, etc.

=== "Rust"
    ```rust
    Rule::new(r"1 - Root rule", MatchRequirement::MustBeFound)
        .extend(vec![
            Rule::new(r"1 - Subrule", MatchRequirement::MustBeFound),
            Rule::new(r"2 - Subrule", MatchRequirement::MustBeFound).extend(vec![
                Rule::new(r"1 - Subrule of subrule", MatchRequirement::MustBeFound),
                Rule::new(r"2 - Subrule of subrule", MatchRequirement::MustBeFound),
            ]),
        ]);
    ```

=== "JavaScript/TypeScript"
    ```js
    new Rule("1 - Root rule", MatchRequirement.MustBeFound)
        .extend([
            new Rule("1 - Subrule", MatchRequirement.MustBeFound),
            new Rule("2 - Subrule", MatchRequirement.MustBeFound)
                .extend([
                    new Rule("1 - Subrule of subrule", MatchRequirement.MustBeFound),
                    new Rule("2 - Subrule of subrule", MatchRequirement.MustBeFound),
                ]),
        ]);
    ```

=== "Python"
    ```python
    Rule(r"1 - Root rule", MatchRequirement.MustBeFound).extend([
        Rule(r"1 - Subrule", MatchRequirement.MustBeFound),
        Rule(r"2 - Subrule", MatchRequirement.MustBeFound).extend([
            Rule(r"1 - Subrule of subrule", MatchRequirement.MustBeFound),
            Rule(r"2 - Subrule of subrule", MatchRequirement.MustBeFound),
        ]),
    ])
    ```