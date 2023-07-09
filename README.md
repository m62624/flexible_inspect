# PYSTVAL

<p align="center">
  <kbd>
    <img src="docs/images/Color logo - no background.svg" alt="Logo" width="700"/>
  </kbd>
</p>



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

The `regex` of rust provides us with `RegexSet`, a data structure which allows us to match a string with a set of regular expressions at the same time. This can be useful when you have a lot of regular expressions and want to quickly determine which ones match a particular string.

The main advantage of using RegexSet is that it can be much faster than applying each regular expression to a string in sequence, especially if you have a large number of regular expressions.

> **The text is taken from the official `rust regex` documentation**\
> For example, consider regular expressions to match email addresses and domains: [a-z]+@[a-z]+\.(com|org|net) and [a-z]+\.(com|org|net). If a regex set is constructed from those regexes, then searching the haystack foo@example.com will report both regexes as matching. Of course, one could accomplish this by compiling each regex on its own and doing two searches over the haystack. The key advantage of using a regex set is that it will report the matching regexes using a single pass through the haystack. If one has hundreds or thousands of regexes to match repeatedly (like a URL router for a complex web application or a user agent matcher), then a regex set can realize huge performance gains. 

### Complex regex

Category rules based on the [**fancy-regex**](https://docs.rs/fancy-regex/latest/fancy_regex/) library. This package supports more complex regular expression functions that may consume more memory. For example, backlinks and forward looking statements may require additional memory to store intermediate results and processing states.

It is important to note that the exact amount of memory consumed will depend on the specific regular expressions and data you are working with.

### Modifier
Modifiers are additional logic changes for `Rule`.