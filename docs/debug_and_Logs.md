## How to debug the code ?

How to understand what the expression caught, what rules worked ? for which rule was the modifier used ? 

The library implements logging modes via an environment variable. 
By default, you always get an error log when you forward an error from rust to python. 

To just see which rule found what matches, you can run a python file with the `info` environment variable

**`RUST_LOG=info file.py`**

**Example log**: 
<details>
<summary>show the log</summary>

```python
# some text

class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)]


test_validator = TemplateValidator([ErrorCheckText])
test_validator.validate(text)
```

```rust
[2023-07-19T05:00:25Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.ErrorCheckText'>` are run
[2023-07-19T05:00:25Z INFO  pystval::rule::next] 
    the result: rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: Single(
        {
            "[1234]",
            "[123]",
            "[123456789]",
        },
    )`,
    
[2023-07-19T05:00:25Z INFO  pystval::rule::next::counter_status] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 4 matches found
[2023-07-19T05:00:25Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:00:25Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work
[2023-07-19T05:00:25Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.ErrorCheckText'>` are run
[2023-07-19T05:00:25Z INFO  pystval::rule::next] 
    the result: rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: Single(
        {
            "[123456789]",
            "[123]",
            "[1234]",
        },
    )`,
    
[2023-07-19T05:00:25Z INFO  pystval::rule::next::counter_status] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 4 matches found
[2023-07-19T05:00:25Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:00:25Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work

```

</details>
<br>

---
If you want to see in addition the use of modifiers and the initialization of the validator

**`RUST_LOG=debug file.py`**

**Example log**:
<details>
<summary>show the log</summary>


```python
# some text

class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)]


test_validator = TemplateValidator([ErrorCheckText])
test_validator.validate(text)
```

```rust
[2023-07-19T05:04:46Z DEBUG pystval::rule::init] rule created (only constructor without modifiers): Rule {
        content: Some(
            TakeRuleForExtend {
                str_with_type: DefaultR(
                    "\\[\\d+\\]",
                ),
                requirement: MustBeFound,
                subrules: None,
                counter: None,
                mod_match: AllRulesForAllMatches,
                duplicate_matches: false,
            },
        ),
    }
[2023-07-19T05:04:46Z DEBUG pystval::rule::modifiers::counter] used the `counter_is_equal` modifier for `Rule` ( `\[\d+\]` )
[2023-07-19T05:04:46Z DEBUG pystval::template_validator] loaded classes in the validator: [
        "<class '__main__.ErrorCheckText'>",
    ]
[2023-07-19T05:04:46Z DEBUG pystval::template_validator::validate] synchronous validator is running
[2023-07-19T05:04:46Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.ErrorCheckText'>` are run
[2023-07-19T05:04:46Z INFO  pystval::rule::next] 
    the result: rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: Single(
        {
            "[1234]",
            "[123456789]",
            "[123]",
        },
    )`,
    
[2023-07-19T05:04:46Z INFO  pystval::rule::next::counter_status] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 4 matches found
[2023-07-19T05:04:46Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:04:46Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work
[2023-07-19T05:04:46Z DEBUG pystval::custom_error] received variables to fill the `<class '__main__.ErrorCheckText'>` message: []
[2023-07-19T05:04:46Z DEBUG pystval::template_validator::validate] synchronous validator is running
[2023-07-19T05:04:46Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.ErrorCheckText'>` are run
[2023-07-19T05:04:46Z INFO  pystval::rule::next] 
    the result: rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: Single(
        {
            "[123]",
            "[1234]",
            "[123456789]",
        },
    )`,
    
[2023-07-19T05:04:46Z INFO  pystval::rule::next::counter_status] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 4 matches found
[2023-07-19T05:04:46Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:04:46Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work
[2023-07-19T05:04:46Z DEBUG pystval::custom_error] received variables to fill the `<class '__main__.ErrorCheckText'>` message: []
```

</details>
<br>

---
If you want to see only errors in the logs

**`RUST_LOG=error file.py`**

**Example log**: 
<details>
<summary>show the log</summary>

```python
# some text

class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)]


test_validator = TemplateValidator([ErrorCheckText])
test_validator.validate(text)
```

```rust
[2023-07-19T05:06:41Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:06:41Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work
[2023-07-19T05:06:41Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:06:41Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work
```

</details>
<br>

---
If you want to see which rule is running from the stack and a full description of the rule with modifications

**`RUST_LOG=trace file.py`**

**Example log**: 
<details>
<summary>show the log</summary>

```python
# some text

class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)]


test_validator = TemplateValidator([ErrorCheckText])
test_validator.validate(text)
```

```rust
[2023-07-19T05:09:44Z DEBUG pystval::rule::init] rule created (only constructor without modifiers): Rule {
        content: Some(
            TakeRuleForExtend {
                str_with_type: DefaultR(
                    "\\[\\d+\\]",
                ),
                requirement: MustBeFound,
                subrules: None,
                counter: None,
                mod_match: AllRulesForAllMatches,
                duplicate_matches: false,
            },
        ),
    }
[2023-07-19T05:09:44Z DEBUG pystval::rule::modifiers::counter] used the `counter_is_equal` modifier for `Rule` ( `\[\d+\]` )
[2023-07-19T05:09:44Z DEBUG pystval::template_validator] loaded classes in the validator: [
        "<class '__main__.ErrorCheckText'>",
    ]
[2023-07-19T05:09:44Z DEBUG pystval::template_validator::validate] synchronous validator is running
[2023-07-19T05:09:44Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.ErrorCheckText'>` are run
[2023-07-19T05:09:44Z TRACE pystval::rule::runner] loading rules from top-level stack : `\[\d+\]`, mode `AllRulesForAllMatches`
[2023-07-19T05:09:44Z TRACE pystval::rule::runner] loading mode AllRulesForAllMatches for the rule `\[\d+\]`
[2023-07-19T05:09:44Z TRACE pystval::rule::runner::context_match::all_rules_for_all_matches] temporary stack created
[2023-07-19T05:09:44Z INFO  pystval::rule::next] 
    the result: rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: Single(
        {
            "[123456789]",
            "[123]",
            "[1234]",
        },
    )`,
    
[2023-07-19T05:09:44Z INFO  pystval::rule::next::counter_status] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 4 matches found
[2023-07-19T05:09:44Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:09:44Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work
[2023-07-19T05:09:44Z DEBUG pystval::custom_error] received variables to fill the `<class '__main__.ErrorCheckText'>` message: []
[2023-07-19T05:09:44Z DEBUG pystval::template_validator::validate] synchronous validator is running
[2023-07-19T05:09:44Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.ErrorCheckText'>` are run
[2023-07-19T05:09:44Z TRACE pystval::rule::runner] loading rules from top-level stack : `\[\d+\]`, mode `AllRulesForAllMatches`
[2023-07-19T05:09:44Z TRACE pystval::rule::runner] loading mode AllRulesForAllMatches for the rule `\[\d+\]`
[2023-07-19T05:09:44Z TRACE pystval::rule::runner::context_match::all_rules_for_all_matches] temporary stack created
[2023-07-19T05:09:44Z INFO  pystval::rule::next] 
    the result: rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: Single(
        {
            "[123456789]",
            "[1234]",
            "[123]",
        },
    )`,
    
[2023-07-19T05:09:44Z INFO  pystval::rule::next::counter_status] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 4 matches found
[2023-07-19T05:09:44Z ERROR pystval::rule::next::counter_status] for the `\[\d+\]` rule must be matched: `Some(
        Only(
            30,
        ),
    )`
    total matches found: `4`
[2023-07-19T05:09:44Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`\[\d+\]`, `MustBeFound`) didn't work
[2023-07-19T05:09:44Z DEBUG pystval::custom_error] received variables to fill the `<class '__main__.ErrorCheckText'>` message: []
```

</details>
<br>

---