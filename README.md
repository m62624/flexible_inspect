# PYSTVAL

<p align="center">
  <kbd>
    <img src="docs/images/Color logo - no background.svg" alt="Logo" width="700"/>
  </kbd>
</p>

- [PYSTVAL](#pystval)
  - [The Library](#the-library)
  - [Quick Look](#quick-look)
  - [Installation](#installation)
  - [How does it all work?](#how-does-it-all-work)
    - [Cartridge](#cartridge)
    - [Rule](#rule)
      - [Root rule](#root-rule)
      - [Subrule](#subrule)
    - [Simple regex](#simple-regex)
    - [Complex regex](#complex-regex)
    - [Modifier](#modifier)
      - [Different situations](#different-situations)
      - [Matching mode](#matching-mode)
  - [Scheme of operation modes](#scheme-of-operation-modes)
  - [`all_rules_for_all_matches`](#all_rules_for_all_matches)
  - [`all_rules_for_at_least_one_match`](#all_rules_for_at_least_one_match)
  - [`at_least_one_rule_for_all_matches`](#at_least_one_rule_for_all_matches)
  - [`at_least_one_rule_for_at_least_one_match`](#at_least_one_rule_for_at_least_one_match)
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

To install the latest version of Pystval, use the following command:
```bash
pip install pystval-version-platform.whl
```

if `Docker` is installed, clone the repository, go to the folder with the project, build the docker image together with the code and run one of the :

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
| MustNotBeFound   | Yes         | Yes                                | Finish processing                        |
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

![](<docs/scheme/scheme.svg>)

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

## License

[MIT License](LICENSE).