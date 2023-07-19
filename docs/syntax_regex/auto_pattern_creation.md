# auto_generate

The `auto_generate` method is a static method of the `Rule` class that generates a regular expression based on the provided text. This can be useful for rapid prototyping of patterns.

```python
# [ [123] [123] [1234] ] [ [123456789] ]

class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule(r"---\s?.+\s?---", MatchRequirement.MustBeFound).extend(
            [
                Rule(r"\[[^\[\]]+\]", MatchRequirement.MustBeFound).extend(
                    [
                        Rule.auto_generate(MatchRequirement.MustBeFound, [
                            "[1234]", "[123]", "[123456789]"])
                    ]).mode_at_least_one_rule_for_at_least_one_match()
            ]
        )
    ]
```

The method returns a new Rule instance containing the generated regular expression and match requirements.

<details>
<summary>show the log</summary>

```bash
[2023-07-19T05:56:55Z DEBUG pystval::rule::generator] regular expression `^\[123(?:4(?:56789)?\]|\])$` is generated based on the obtained data: 
     [
        "[1234]",
        "[123]",
        "[123456789]",
    ]
```

</details>
</br>

