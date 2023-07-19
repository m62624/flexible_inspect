# PYSTVAL
[![codecov](https://codecov.io/gh/m62624/pystval/branch/main/graph/badge.svg?token=S8H7J5O0AL)](https://codecov.io/gh/m62624/pystval)
[![changelog](https://img.shields.io/badge/changelog-blue)](https://github.com/m62624/pystval/blob/main/CHANGELOG.md)
[![Documentation Status](https://readthedocs.org/projects/pystval/badge/?version=latest)](https://pystval.readthedocs.io/en/latest/?badge=latest)
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
  - [Documentation](#documentation)
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
## Documentation
Documentation of use: [docs](http://pystval.readthedocs.io/)

## License

[MIT License](https://github.com/m62624/pystval/blob/main/LICENSE).
