# PYSTVAL
[![codecov](https://codecov.io/gh/m62624/pystval/branch/main/graph/badge.svg?token=S8H7J5O0AL)](https://codecov.io/gh/m62624/pystval)
[![changelog](https://img.shields.io/badge/changelog-blue)](https://github.com/m62624/pystval/blob/main/CHANGELOG.md)
[![CI/CD Pipeline](https://github.com/m62624/pystval/actions/workflows/github-actions.yml/badge.svg?branch=main)](https://github.com/m62624/pystval/actions/workflows/github-actions.yml)
![PyPI - Downloads](https://img.shields.io/pypi/dm/pystval)

<p align="center">
  <kbd>
    <img src="https://github.com/m62624/pystval/blob/main/docs/images/Color%20logo%20-%20no%20background.png" alt="Logo" width="700"/>
  </kbd>
</p>

- [PYSTVAL](#pystval)
  - [The Library](#the-library)
  - [Quick Look](#quick-look)
  - [Installation](#installation)
    - [PyPi.](#pypi)
    - [Github packages](#github-packages)
    - [Build via Docker](#build-via-docker)
  - [How does it all work?](#how-does-it-all-work)
    - [Cartridge](#cartridge)
    - [Rule](#rule)
      - [Root rule](#root-rule)
      - [Subrule](#subrule)
    - [Syntax regex](#syntax-regex)
    - [Simple regex](#simple-regex)
    - [Complex regex](#complex-regex)
    - [Modifier](#modifier)
      - [Different situations](#different-situations)
      - [Extend rule](#extend-rule)
      - [Matching mode](#matching-mode)
  - [Scheme of operation modes](#scheme-of-operation-modes)
  - [`all_rules_for_all_matches`](#all_rules_for_all_matches)
  - [`all_rules_for_at_least_one_match`](#all_rules_for_at_least_one_match)
  - [`at_least_one_rule_for_all_matches`](#at_least_one_rule_for_all_matches)
  - [`at_least_one_rule_for_at_least_one_match`](#at_least_one_rule_for_at_least_one_match)
      - [Matching text counter](#matching-text-counter)
  - [counter\_is\_equal `X`](#counter_is_equal-x)
  - [counter\_more\_than `X`](#counter_more_than-x)
  - [counter\_less\_than `X`](#counter_less_than-x)
  - [Filling in the error message variables](#filling-in-the-error-message-variables)
  - [How to debug the code ?](#how-to-debug-the-code-)
  - [License](#license)


## The Library
Pystval is a powerful Rust library for text validation and analysis. It allows you to create custom errors with modifiers and define sub-corrections for each rule. With Pystval, you can build versatile validators that handle any text and enforce specific requirements. Customize error behavior and apply sub-corrections based on specific cases or conditions. Pystval empowers you to create accurate and adaptable validators for various text processing needs.

The difference between this library and normal use of regular expressions is that you can combine different rules with modifiers, and the results of these rules are automatically passed to subexpressions, etc.

## Quick Look
<details>
<summary>Click me</summary>

```python
from pystval import Rule, MatchRequirement, TemplateValidator, PystvalException

# Creating a validator cartridge
class ErrorInvalidFormat(PystvalException):
    message = ":: Invalid Format ::"
    rules = [
        # root rule
        Rule(r"(?i)abc.+\d+", MatchRequirement.MustBeFound).extend([
            # 1-subrule of root rule
            Rule(r"\d{3}-\d{4}-\d{2}", MatchRequirement.MustBeFound).extend([
                # 1-subrule of 1-subrule
                Rule(r"^\d{3}", MatchRequirement.MustBeFound).extend(
                    # 1-subrule of 1-subrule of 1-subrule
                    [Rule(r"[0-1][1-2][1-3]", MatchRequirement.MustBeFound)]),
                # 2-subrule of 1-subrule
                Rule(r"-", MatchRequirement.MustBeFound),
            ]),
        ])
    ]

# Creating a validator cartridge
# We get this class as an error because `MustNotBeFound` is specified, but we find in the text, at the very end of `12345`
class ErrorNumber(PystvalException):
    message = "Custom error with value : {num}"
    rules = [
        Rule(r"(?<num>\d+(?!\d|-|\s))", MatchRequirement.MustNotBeFound)
    ]


text = b"This is a complex text with aBc_122-4567-99 and def. The number is 12345"

# In the validator we load the cartridges, which will be used to check the data
validator = TemplateValidator([ErrorInvalidFormat, ErrorNumber])

# We get a list of errors based on those classes whose rules failed
list_error = validator.validate(text)

# errors
if list_error is not None:
    for error in list_error:
        try:
            raise error
        except PystvalException as e:

            # show a specific message based on the class
            print(e.report)
```
> OUTPUT: **Custom error with value : 12345**

</details>

## Installation

### PyPi.
Installation from `PyPi`

```bash
pip install pystval
```

### Github packages

Under `Releases` choose your platform.
Unpack the archive and install

```bash
pip install pystval-version-platform.whl
```

### Build via Docker

If `Docker` is installed, clone the repository, navigate to the project folder, build the docker image together with the code and run the container together with one of the :

```bash
# for Linux
make linux-amd
make linux-arm

# for windows
make windows-amd
make windows-arm

# for macOS
make mac-amd
make mac-arm

```
**Supported Platforms**:

|                     | **Linux**                   | **Windows**               | **macOS**              |
| ------------------- | --------------------------- | ------------------------- | ---------------------- |
| System architecture | `x86_64-unknown-linux-gnu`  | `x86_64-pc-windows-msvc`  | `aarch64-apple-darwin` |
|                     | `aarch64-unknown-linux-gnu` | `aarch64-pc-windows-msvc` | `x86_64-apple-darwin`  |




## How does it all work? 
Before explaining how the library works, let's understand the terminology.
We have such concepts as : 
- `Cartridge`
- `Rule`
- `Root rule`
- `Subrule`
- `Simple regex`
- `Complex regex`
- `Modifier`


### Cartridge
A `cartridge` is a structure that contains a class for creating an error, class rules by which the check is done, and a message with a possible valid value. For `Python`, the `cartridge` is your error class inherited from `PystvalException`. Based on different cartridges you can create validators with different validation conditions.

### Rule
The `rule` is a structure for storing a regular expression with modifiers. Structure is the basic minimum unit of the validation logic

```python
 Rule(r"\d+", MatchRequirement.MustNotBeFound)
```

#### Root rule
The `root rule` - all rules that are in the first step of the `cardridge`, and is also the root in relation to the `subrule``

```python
class CustomError_1(PystvalException):
    message = "my custom error 1"
    rules = [
        Rule(r"1 - Root rule", MatchRequirement.MustBeFound),
        Rule(r"2 - Root rule", MatchRequirement.MustBeFound),
        Rule(r"3 - Root rule", MatchRequirement.MustBeFound),
    ]
```

#### Subrule
The `subrule` is the rules that are below the first step of the cartridge, as well as all rules created inside the `extend` method.
> But then again, even if `subrule` (**A**) is created within extend, for all `subrule` (**B** of **A**), `subrule` A itself will be the `root` for them

```python
class CustomError_2(PystvalException):
    message = "my custom error 2"
    rules = [
        Rule(r"1 - Root rule", MatchRequirement.MustBeFound).extend([
            Rule(r"1 - Subrule", MatchRequirement.MustBeFound),
            Rule(r"2 - Subrule", MatchRequirement.MustBeFound).extend([
                Rule(r"1 - Subrule of subrule", MatchRequirement.MustBeFound),
                Rule(r"2 - Subrule of subrule", MatchRequirement.MustBeFound),
            ]),
            Rule(r"3 - Subrule", MatchRequirement.MustBeFound),
        ]),
        Rule(r"2 - Root rule", MatchRequirement.MustBeFound),
        Rule(r"3 - Root rule", MatchRequirement.MustBeFound),
    ]
```
### Syntax regex
Since all calculations using regex take place in `Rust`, it is necessary to follow the format `rust regex`.

More information on syntax :
- [Default-Regex](https://docs.rs/regex/latest/regex/#syntax)
- [Fancy-Regex](https://docs.rs/fancy-regex/latest/fancy_regex/#syntax)
- [the same as those links, but with only a syntax table ](https://github.com/m62624/pystval/blob/main/docs/syntax_regex/regex.md)

### Simple regex
Category rules based on the [**regex**](https://docs.rs/regex/latest/regex/) library.
This package is optimized for fast execution and low memory consumption. It uses efficient algorithms and data structures to minimize memory usage while processing regular expressions.

The `regex` of rust provides us with [`RegexSet`](https://docs.rs/regex/latest/regex/struct.RegexSet.html), a data structure which allows us to match a string with a set of regular expressions at the same time. This can be useful when you have a lot of regular expressions and want to quickly determine which ones match a particular string.

The main advantage of using RegexSet is that it can be much faster than applying each regular expression to a string in sequence, especially if you have a large number of regular expressions.

> **The text is taken from the official `rust regex` documentation**\
> For example, consider regular expressions to match email addresses and domains: [a-z]+@[a-z]+\.(com|org|net) and [a-z]+\.(com|org|net). If a regex set is constructed from those regexes, then searching the haystack foo@example.com will report both regexes as matching. Of course, one could accomplish this by compiling each regex on its own and doing two searches over the haystack. The key advantage of using a regex set is that it will report the matching regexes using a single pass through the haystack. If one has hundreds or thousands of regexes to match repeatedly (like a URL router for a complex web application or a user agent matcher), then a regex set can realize huge performance gains. 

### Complex regex

Category rules based on the [**fancy-regex**](https://docs.rs/fancy-regex/latest/fancy_regex/) library. This package supports more complex regular expression functions that may consume more memory. For example, backlinks and forward looking statements may require additional memory to store intermediate results and processing states.

It is important to note that the exact amount of memory consumed will depend on the specific regular expressions and data you are working with.

### Modifier
Modifiers are additional logic changes for `Rule`. Modifiers are an important element of the `rules`.  When you create a `Rule` there are always at least two modifiers created. 

The first one is the category of the pattern. The `Simple regex` or `Complex regex` is a hidden modifier, it cannot be called for the `Rule`, but it is defined based on the pattern. When you create a regular expression without leading and trailing checks, it will be put into the category `Simple Rule`, and will be used in the `RegexSet` for each text. If you create a regular expression with leading and trailing checks, the rule goes to the end of the queue. So we go through an easy regular expression at the beginning and a hard one at the end.

The second modifier is a kind of conditional operator. 
- `MustBeFound` - which means we must necessarily get a match from this regular expression
- `MustNotBeFound` - which means, based on this regular expression, we must not get a match 

| MatchRequirement | Match found | does this rule have any subrules ? | Result                                   |
| ---------------- | ----------- | ---------------------------------- | ---------------------------------------- |
| MustBeFound      | Yes         | Yes                                | Continue processing subrules             |
| MustBeFound      | Yes         | No                                 | Finish processing                        |
| MustBeFound      | No          | Yes                                | Error (match must have been found)       |
| MustBeFound      | No          | No                                 | Error (match must have been found)       |
| MustNotBeFound   | Yes         | Yes                                | Continue processing subrules             |
| MustNotBeFound   | Yes         | No                                 | Error (match should not have been found) |
| MustNotBeFound   | No          | Yes                                | Finish processing                        |
| MustNotBeFound   | No          | No                                 | Finish processing                        |

For example, we have a regular expression `r"\d+"` and we want to get a match from it. We create a rule with the modifier `MustBeFound`. If we get a match, we continue to process the subrules. If we don't get a match, we get an error.

```bash
#=======================================
text = "txt txt txt 910 301 44 text"
#=======================================

CustomError
|
root rule : r"\d+" with modifier MustBeFound
   |     
   |___ if true,true -> new captures from root: 910, 301, 44,
         |__ subrule from root rule : "\d{2}" with modifier MustBeFound
               |__ if true,true -> new captures from subrule: 44,
                  |__  ... 
                     |__   ...
                        |__   ...
```
#### Different situations 

As you may have noticed, there is a difference between these two options: 

| MatchRequirement   | Match found | does this rule have any subrules ? | Result                                   |
| ------------------ | ----------- | ---------------------------------- | ---------------------------------------- |
| **MustNotBeFound** | **Yes**     | **Yes**                            | **Continue processing subrules**         |
| MustNotBeFound     | Yes         | No                                 | Error (match should not have been found) |

This is done so that if you should not find this, but you do find it, you can create a subrules for additional checks with modifiers. If nothing is found, the subcorrections will simply be skipped.

#### Extend rule

Modification to extend the rule with subrules. This is a very important modifier, because it allows you to create a tree of rules, and also allows you to create a tree of rules inside a tree of rules, etc.

```python
Rule(r"1 - Root rule", MatchRequirement.MustBeFound).extend([
    Rule(r"1 - Subrule", MatchRequirement.MustBeFound),
    Rule(r"2 - Subrule", MatchRequirement.MustBeFound).extend([
        Rule(r"1 - Subrule of subrule", MatchRequirement.MustBeFound),
        Rule(r"2 - Subrule of subrule", MatchRequirement.MustBeFound),
    ]),
])
```

#### Matching mode
Before we looked at modifiers that affect one `Rule`, but now we will study modifiers that affect all `subrules` within one `root rule`

- `all_rules_for_all_matches`
- `all_rules_for_at_least_one_match`
- `at_least_one_rule_for_all_matches`
- `at_least_one_rule_for_at_least_one_match`

Scheme of operation modes
---

<details>
<summary>Click me</summary>

![](<https://github.com/m62624/pystval/blob/main/docs/scheme/scheme.svg>)

</details>

`all_rules_for_all_matches` 
---

In this mode, to which all additional rules apply. (default mode for everyone)
We check that for each match (text) all the rules will work.

```bash
#=======================================
text = "txt [123] txt [456] txt [789]"
#=======================================

CustomError
|
|__ Rule "\[[^\[\]]+\]" (MustBeFound)
     |   [123], [456], [789]
     |___ Subrule ".+" (MustBeFound) ---> [123] -> [456] -> [789] -- TRUE 
     |                                      |       |        |
     |___ Subrule "\[\d+\]" (MustBeFound) __|_______|________|
```

`all_rules_for_at_least_one_match`
---

In this mode, all the sub-adjustments should work for at least one match. If at least one sub-rule does not work on one of the matches, an error will be returned.

```bash
#=======================================
text = "txt [123] txt [456] txt [789]"
#=======================================

CustomError
|
|__ Rule "\[[^\[\]]+\]" (MustBeFound)
    |   [123], [456], [789]
    |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE 
    |                                      |
    |___ Subrule "\[\d+\]" (MustBeFound) __|
    |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- ERROR

```

`at_least_one_rule_for_all_matches` 
---

In this mode, at least one sub-rule should work for every match. If no sub-rule works on one of the matches, an error will be returned.

```bash
#=======================================
text = "txt [123] txt [456] txt [789]"
#=======================================

CustomError
|
|__ Rule "\[[^\[\]]+\]" (MustBeFound)
    |   [123], [456], [789]
    |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE -- [456] -- TRUE -- [789] -- TRUE
    |                                      |               |                 |
    |___ Subrule "\[\d+\]" (MustBeFound) __|_______________|_________________|
    |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- TRUE (since other rules matched)

```

`at_least_one_rule_for_at_least_one_match`
--- 

In this mode, at least one sub-rule should work for at least one match. If no sub-rule works on one of the matches, an error will be returned.

```bash
#=======================================
text = "txt [123] txt [456] txt [789]"
#=======================================

CustomError
|
|__ Rule "\[[^\[\]]+\]" (MustBeFound)
    |   [123], [456], [789]
    |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE 
    |                                      |
    |___ Subrule "\[\d+\]" (MustBeFound) __|
    |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- TRUE (since other rules matched for at least one match)

```

#### Matching text counter

Before we see the following modifiers, let's see how text match capture works when a rule is triggered, for example pattern `\d+`, for text `123 123 123 54 6 7 8`. We only get `123`, `6`, `7`, `8`, what happened now?, for matches in the library we use `HashSet<&'s str>`, this is necessary so as not to check once again all the rules of three matches `123`, BUT we always keep the number of identical matches. By doing this, we save only unique values and keep their counter so that we always know how many times they are repeated

counter_is_equal `X`
---
Adding a match counter, where the condition is: 
there must be exactly `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)
```

counter_more_than `X`
---
Adding a match counter, where the condition is: 
there must be greater than or equal to `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_more_than(12)
```

counter_less_than `X`
---
Adding a match counter, where the condition is: 
there must be less than or equal to `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_less_than(1000)
```

## Filling in the error message variables

Before we continue, let's find out how it works when you put the same names in the error messages and in Rule with the modifier `MustNotBeFound`, we fill in the message with the data we got from the rule and here we need to remember, no matter how many groups you create in one regular expression, we save data from the whole regular expression 

```python
text = "123123sd"
Rule(r"(?P<NUMBER_3>\d{3})(?P=NUMBER_3)\w+", MatchRequirement.MustBeFound)
```
> OUTPUT: 
```rust
Some(Captures({
    0: Some("123123sds"),
    1: Some("123"),
})),
```

we skip the match from the `NUMBER_3` (**1**) subgroup, we keep the overall result from the expression (**0**).

---
When you specify the `message` attribute in a class
 
```python
# some code
message = "error message with value {INFO}"
# some code
```
you can specify the same name as the name of the capture group in a regular expression

```python
 Rule(r"(?P<INFO>\d+)", MatchRequirement.MustNotBeFound)
```

If you want to output only one variable, you can use the reserved name without assigning a group name to the regular mode 
`main_capture`.

```python
# some code
message = "error message with value {main_capture}"
# some code
 Rule(r"\d+", MatchRequirement.MustNotBeFound)
```

## How to debug the code ?

How to understand what the expression caught, what rules worked ? for which rule was the modifier used ? 

The library implements logging modes via an environment variable. 
By default, you always get an error log when you forward an error from rust to python. 

To just see which rule found what matches, you can run a python file with the `info` environment variable

**`RUST_LOG=info file.py`**

**Example log**: 
<details>
<summary>Click me</summary>

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

If you want to see in addition the use of modifiers and the initialization of the validator

**`RUST_LOG=debug file.py`**

**Example log**:
<details>
<summary>Click me</summary>

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

If you want to see only errors in the logs

**`RUST_LOG=error file.py`**

**Example log**: 
<details>
<summary>Click me</summary>

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

If you want to see which rule is running from the stack and a full description of the rule with modifications

**`RUST_LOG=trace file.py`**

**Example log**: 
<details>
<summary>Click me</summary>

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

## License

[MIT License](https://github.com/m62624/pystval/blob/main/LICENSE).
