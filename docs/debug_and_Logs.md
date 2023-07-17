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

class TestError1(PystvalException):
    message = "error message {main_capture}"
    rules = [
        Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)]

test_validator = TemplateValidator([TestError1])
test_validator.validate(text)
```

```bash
[2023-07-09T15:02:54Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.TestError1'>` are run
[2023-07-09T15:02:54Z INFO  pystval::rule::runner] rule processing mode `\[\d+\]` : `all_rules_for_all_matches`
[2023-07-09T15:02:54Z INFO  pystval::rule::next] 
    THE RESULT: 
    rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: {
        "[1]",
    }`,
    
[2023-07-09T15:02:54Z INFO  pystval::rule::next] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 30 matches found
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

class TestError1(PystvalException):
    message = "error message {main_capture}"
    rules = [
        Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)]

test_validator = TemplateValidator([TestError1])
test_validator.validate(text)
```

```bash
[2023-07-09T15:28:09Z DEBUG pystval::rule::init] rule created (only constructor without modifiers): Rule {
        content: Some(
            TakeRuleForExtend {
                str_with_type: DefaultR(
                    "\\[\\d+\\]",
                ),
                requirement: MustBeFound,
                subrules: None,
                counter: None,
                mod_match: AllRulesForAllMatches,
            },
        ),
    }
[2023-07-09T15:28:09Z DEBUG pystval::rule::modifiers] used the `counter_is_equal` modifier for `Rule` ( `\[\d+\]` )
[2023-07-09T15:28:09Z DEBUG pystval::template_validator] loaded classes in the validator: [
        "<class '__main__.TestError1'>",
    ]
[2023-07-09T15:28:09Z DEBUG pystval::template_validator::validate] synchronous validator is running
[2023-07-09T15:28:09Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.TestError1'>` are run
[2023-07-09T15:28:09Z INFO  pystval::rule::runner] rule processing mode `\[\d+\]` : `all_rules_for_all_matches`
[2023-07-09T15:28:09Z INFO  pystval::rule::next] 
    THE RESULT: 
    rule: (`\[\d+\]`, `MustBeFound`),
    `Captures: {
        "[1]",
    }`,
    
[2023-07-09T15:28:09Z INFO  pystval::rule::next] 
    THE RESULT: 
    rule: `\[\d+\]`,
    rule counter Some(
        Only(
            30,
        ),
    ),
    a total of 30 matches found
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

class TestError1(PystvalException):
    message = "error message {main_capture}"
    rules = [
        Rule(r"textWWW", MatchRequirement.MustBeFound).counter_is_equal(30)]

test_validator = TemplateValidator([TestError1])
test_validator.validate(text)
```

```bash
[2023-07-09T15:32:01Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`textLA`, `MustBeFound`) didn't work
error message ___
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

class TestError1(PystvalException):
    message = "error message {main_capture}"
    rules = [
        Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)]

test_validator = TemplateValidator([TestError1])
test_validator.validate(text)
```

```bash
[2023-07-09T15:34:06Z DEBUG pystval::rule::init] rule created (only constructor without modifiers): Rule {
        content: Some(
            TakeRuleForExtend {
                str_with_type: DefaultR(
                    "textLA",
                ),
                requirement: MustBeFound,
                subrules: None,
                counter: None,
                mod_match: AllRulesForAllMatches,
            },
        ),
    }
[2023-07-09T15:34:06Z DEBUG pystval::rule::modifiers] used the `counter_is_equal` modifier for `Rule` ( `textLA` )
[2023-07-09T15:34:06Z DEBUG pystval::template_validator] loaded classes in the validator: [
        "<class '__main__.TestError1'>",
    ]
[2023-07-09T15:34:06Z DEBUG pystval::template_validator::validate] synchronous validator is running
[2023-07-09T15:34:06Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.TestError1'>` are run
[2023-07-09T15:34:06Z INFO  pystval::rule::runner] rule processing mode `textLA` : `all_rules_for_all_matches`
[2023-07-09T15:34:06Z TRACE pystval::rule::runner::context_match::all_rules_for_all_matches] started rule (`textLA`, `MustBeFound`) from the stack
    Full details of the rule (after modifications): Rule {
        content: Some(
            TakeRuleForExtend {
                str_with_type: DefaultR(
                    "textLA",
                ),
                requirement: MustBeFound,
                subrules: None,
                counter: Some(
                    Only(
                        30,
                    ),
                ),
                mod_match: AllRulesForAllMatches,
            },
        ),
    }
[2023-07-09T15:34:06Z INFO  pystval::rule::next] 
    THE RESULT: 
    rule: (`textLA`, `MustBeFound`),
    `Captures: {}`,
    
[2023-07-09T15:34:06Z ERROR pystval::rule::runner::context_match::all_rules_for_all_matches] the rule (`textLA`, `MustBeFound`) didn't work
[2023-07-09T15:34:06Z DEBUG pystval::custom_error] received variables to fill the `<class '__main__.TestError1'>` message: [
        "main_capture",
    ]
error message ___
```

</details>
<br>

---