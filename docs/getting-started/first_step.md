Now we can start writing the code. Let's start with importing

```python
from pystval import Rule, MatchRequirement, TemplateValidator, PystvalException
```

Next, let's look at the text for which validation will take place 

text :
```python
text = b"""
Hi! 🌞
---
This is an example of text with different nesting of characters. It has regular letters, numbers, punctuation marks, as well as special characters, emoji and even Unicode characters. Some characters can be nested within each other, such as quotation marks " " or parentheses ( ) [ [123] [123] [1234] ]. Special characters such as the tilde ~ or the dollar sign $ can also be used.
--- 

Validation of text with different nesting of characters may include checking for the presence of paired characters (as in the case of quotes or brackets), correct use of special characters, and compliance with specified rules. For example, if there is an initial quotation mark in the text, there must be a corresponding final quotation mark.

Such validation can be useful, for example, when processing user input, analyzing text, or checking formatting.

I hope this example has helped you visualize how text with different nesting of characters can be used for validation.
"""
```

Suppose that in the first step we want to check that the square brackets are inside the characters

> `"---"`

But for ease of understanding, first we just search only the text that is included in `---`

Let's create a class called `ErrorCheckText`, the class should inherit `PystvalException`. This is necessary so that we can catch errors from the validator and receive the message via `report`.

```py
class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule("---\s?.+\s?---", MatchRequirement.MustBeFound)
    ]
```

When creating an `ErrorCheckText` cartridge, it is mandatory to specify the message and add at least one rule for validation. When creating a rule at least three modifiers are created for each rule, one of which is `MatchRequirement`, after the second is a hidden modifier that defines the type of regex (default regex or fancy regex), it cannot be explicitly changed, it is determined based on what syntax you used (default regex or fancy regex) and the third modifier is the match validation mode modifier.

`MatchRequirement`` is a kind of conditional operator ([more information about this modifier]).

- `MustBeFound` - which means we must necessarily get a match from this regular expression
- `MustNotBeFound` - which means, based on this regular expression, we must not get a match


First, let's create the validator, we can load the cartridge into the validator

```py
simple_text_validator = TemplateValidator([CheckErrorText])
```
for the basic example we use synchronous version of validations, returns `Option List [error error erorr]`

```py
result = simple_text_validator.validate(text)
```

Checking the result of validations

```py
if result is None:
    print("text is valid")
else:
    for error in simple_text_validator.validate(text):
        try:
            raise error
        except PystvalException as e:
            print(error.report)
```

Before we run the code, let's talk about logging. Since the library is written in rust through a simple debug, you will not see in a clear way how each step in the code occurs. That's why the library implements logs, to use logs, use the `RUST_LOG` environment variable.

To just see which rule found what matches, you can run a python file with the `info` environment variable

```bash
RUST_LOG=info python3 main.py
```
<details>
<summary>Show log</summary>

```sh
[2023-07-17T06:01:13Z INFO  pystval::cartridge::runner] all rules of the `<class '__main__.CheckErrorText'>` are run
[2023-07-17T06:01:13Z INFO  pystval::rule::runner] rule processing mode `---\s?.+\s?---` : `all_rules_for_all_matches`
[2023-07-17T06:01:13Z INFO  pystval::rule::next] 
    THE RESULT: 
    rule: (`---\s?.+\s?---`, `MustBeFound`),
    `Captures: {
        "---\nThis is an example of text with different nesting of characters. It has regular letters, numbers, punctuation marks, as well as special characters, emoji and even Unicode characters. Some characters can be nested within each other, such as quotation marks \" \" or parentheses ( ) [ [123] [123] [1234] ] [ [123456789] ]. Special characters such as the tilde ~ or the dollar sign $ can also be used.\n---",
    }`,
    
text is valid
```
</details>
<br>
With the help of logs, you can check which rules have passed the conditions and which have not, what their modifiers are used and the moment of their initializations ([more log information](../debug_and_Logs.md))

Now knowing how to use logs, we can continue writing rules for validations. We got only the text that is included in `---`, now we want to get for example only brackets with any other values except brackets themselves, i.e. without nesting. 

For this we can use the extend modifier, when you create root rules, they must all fulfill their condition or else throw an error. If you want some rule to work for some match. You must create a sub-rule using `extend`. 

```python
class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule("---\s?.+\s?---", MatchRequirement.MustBeFound).extend(
            [
                Rule("[[^\[\]]+\]",MatchRequirement.MustBeFound)
            ]
        )
    ]
```

So, the main difference between a root rule and a subrule is that all roots must fulfill the condition, when subrules can have different checking modes. By standard, when you create subrules, they have : `all_rules_for_all_matches` enabled by default, but you can change it to 

- `all_rules_for_at_least_one_match`
- `at_least_one_rule_for_all_matches`
- `at_least_one_rule_for_at_least_one_match`

Now let's take a look at the standard mode. In this mode, to which all additional rules apply. (default mode for everyone)
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

Since we only have one tweak right now, we may not change that mode. The moment the root rule found a match, all matches are passed as new text to subrules 

That is, in our example, we first got the desired fragment from the whole text, and from that fragment we got `[text] [text] [text] [text]`

