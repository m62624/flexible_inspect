"""
The `Flexible_inspect` is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\\
The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.
"""

import enum
from collections.abc import Iterator
from typing import Optional, Type, List, Any, TypeVar, Union


# ========================================================

def init_logger_with_offset(hour_offset: int):
    """
    Initialization with time zone offset. 
    Function does not have to declare the logging work. 
    It is necessary for logging time offset.
    To run logging, specify an environment variable with the desired logging level
    ```bash
    FLEX_VALIDATOR_LOG=DEBUG "command to run the code"
    ```

    Parameters
    ----------
    `hour_offset` - offset from UTC time (to offset the time, initialize earlier than the rules)
    """
    ...


class MatchRequirement(enum.Enum):
    r"""
The structure that defines what action is required when finding regular expression matches.

## MatchRequirement
 - `MustBeFound` - which means we must necessarily get a match from this regular expression
 - `MustNotBeFound` - which means, based on this regular expression, we must not get a match

Behavior of `Rule` | `RuleBytes` under different conditions

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

    """
    MustBeFound = 0
    """
    The match must be found
    """
    MustNotBeFound = 1
    """
    the match must not be found
    """
# ========================================================


# ========================================================
class Rule:
    """
 A rule is the minimum unit of logic in a validator.
 The rule supports two regular expression crates:
 [**Regex**](https://crates.io/crates/regex) and [**FancyRegex**](https://crates.io/crates/fancy-regex).
 Determines which type is used based on the syntax (for example, if Lookahead and Lookbehind references are used, this automatically defines as [**FancyRegex**](https://crates.io/crates/fancy-regex)).

 The most important feature is that the rule is recursive (don't worry, recursion is not used here).
 Each rule can have nested rules, and these nested rules can have their own nested rules, and so on.
 Thus, when the root rule is triggered, all the results obtained are passed to the nested rules, so you can build complex structural rules to suit any taste

 # Notes
 * Remember any modifier takes the contents of the `Rule` body
 and returns a new one with a changed parameter (only `None` from the original Rule remains),
 so specify the modifier in the same place where you initialize `Rule`.
 * If you stick with the [**Regex**](https://crates.io/crates/regex) library features, all root and nested rules go into [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html).
 Many expressions can be accommodated in a regular expression without *Lookahead* and *Lookbehind* references.
 But this is just a recommendation. If you need to use references, of course you can specify them.
 Then these rules will not be included in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html),
 and if there are rules in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html) they will be the first in the queue to be checked, and those that use [**FancyRegex**](https://crates.io/crates/fancy-regex) features will be at the end of the queue
 * Basically use `Rule` instead of `RuleBytes` when working with text (not necessarily just text, it also includes `html` structures, code fragments from other languages, etc.) since it has support for [**Regex**](https://crates.io/crates/regex) and [**FancyRegex**](https://crates.io/crates/fancy-regex).
 * How is recursive structure checking performed without recursion?
 Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion

    """

    def __init__(self, pattern: str, requirement: MatchRequirement) -> None:
        r"""
 Constructor for creating `Rule`

 # Notes:
     * Please stick to *raw string literals* when creating regular expressions, without it your regular expression may behave differently\
         > `r"d{3}."` - is the correct conversion to a regular expression\
         > `"d{3}."` - possible incorrect behavior

    by default, `all_rules_for_all_matches`. In this mode, all rules must be tested for all matches
        """
        ...

    def extend(self, nested_rules: List[Rule]) -> Rule:
        """
    Extend the rule with nested rules.

    By default, `all_rules_for_all_matches`. In this mode, all rules must be tested for all matches

    Parameters
    ----------
    `nested_rules` : `List[Rule]`
        The nested rules to be added

    Raises
    ------
    `TypeError`
        If `nested_rules` is not a list of `Rule` objects

        """
    ...

    def counter_is_equal(self, count: int) -> Rule:
        """
    modifier to set the match counter, condition `counter == match`
        """
    ...

    def counter_more_than(self, count: int) -> Rule:
        """
    modifier to set the match counter, condition `counter >= match`
        """
    ...

    def counter_less_than(self, count: int) -> Rule:
        """
    modifier to set the match counter, condition `counter <= match`
        """
    ...

    def all_r_for_any_m(self) -> Rule:
        """
        modifier to change the rule matching mode.

        In this mode, all rules must pass the test for at least one match
         """
    ...

    def any_r_for_all_m(self) -> Rule:
        """
        modifier to change the rule matching mode.

        In this mode, at least one rule must pass the test for all matches.
        """
    ...

    def any_r_for_any_m(self) -> Rule:
        """
        modifier to change the rule matching mode.

        In this mode, at least one rule must pass at least one match check
        """
    ...
# ========================================================


# ========================================================
class RuleBytes:
    """
    A rule is the minimum unit of logic in a validator
    (recommendation to use the string version if possible. More information on `Rule`.)

    The most important feature is that the rule is recursive (don't worry, recursion is not used here).
    Each rule can have nested rules, and these nested rules can have their own nested rules, and so on.
    Thus, when the root rule is triggered, all the results obtained are passed to the nested rules, so you can build complex structural rules to suit any taste

    # Notes
    * Remember any modifier takes the contents of the `RuleBytes` body
    and returns a new one with a changed parameter (only `None` from the original Rule remains),
    so specify the modifier in the same place where you initialize `RuleBytes`.
    * Use `&[u8]` when searching for regex matches in haystacks. ([**FancyRegex**](https://crates.io/crates/fancy-regex) capabilities are not available)
    * Unicode support can be disabled, even if disabling it will result in a match with invalid `UTF-8` bytes. More info at [link](https://docs.rs/regex/latest/regex/bytes/index.html)
    * How is recursive structure checking performed without recursion?
    Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion
    """

    def __init__(self, pattern: str, requirement: MatchRequirement) -> None:
        r""" 
    Constructor for creating `RuleBytes`
    # Notes
     * Please stick to *raw string literals* when creating regular expressions, without it your regular expression may behave differently
     ## Example
        > `r"d{3}."` - is the correct conversion to a regular expression\
        >  `"d{3}."` - possible incorrect behavior

        by default, `all_rules_for_all_matches`. In this mode, all rules must be tested for all matches
        """
        ...

    def extend(self, nested_rules: List[RuleBytes]) -> RuleBytes:
        """
    Extend the rule with nested rules.

    By default, `all_rules_for_all_matches`. In this mode, all rules must be tested for all matches

    Parameters
    ----------
    `nested_rules` : `List[RuleBytes]`
        The nested rules to be added

    Raises
    ------
    `TypeError`
        If `nested_rules` is not a list of `RuleBytes` objects

        """
    ...

    def counter_is_equal(self, count: int) -> RuleBytes:
        """
    modifier to set the match counter, condition `counter == match`
        """
    ...

    def counter_more_than(self, count: int) -> RuleBytes:
        """
    modifier to set the match counter, condition `counter >= match`
        """
    ...

    def counter_less_than(self, count: int) -> RuleBytes:
        """
    modifier to set the match counter, condition `counter <= match`
        """
    ...

    def all_r_for_any_m(self) -> Rule:
        """
        modifier to change the rule matching mode.

        In this mode, all rules must pass the test for at least one match
         """
    ...

    def any_r_for_all_m(self) -> Rule:
        """
        modifier to change the rule matching mode.

        In this mode, at least one rule must pass the test for all matches.
        """
    ...

    def any_r_for_any_m(self) -> Rule:
        """
        modifier to change the rule matching mode.

        In this mode, at least one rule must pass at least one match check
        """
    ...
# ========================================================


# ========================================================
class Cartridge:
    """
 The cartridge is the container of the rules.

 # Notes
 * Use a container for one object if possible. Imagine that one container is one specific error `NotFound`, `InvalidHeader`, `WrongCase`.
    """

    def __init__(self, id: int, message: str, root_rules: List[Rule]) -> None:
        r"""

    Parameters
    ----------
    `id` : `int`
            Error code
    `message` : `str`
            Error message
    `root_rules` : `List[Rule]`
            The rules to be added

    Constructor for `Cartridge`

    # Notes

    By default, `all_rules_for_all_matches`. In this mode, all rules must be tested for all matches

    ## Fill in messages
     * Each cartridge supports filling the message with unwanted data, when specifying a message,
     you can specify a variable in the message in the format : **`{variable}`**.
     After specifying an identical group name in any rule along with the *`MustNotBeFound`* modifier
        """
        ...

    def any_r_for_any_m(self) -> Cartridge:
        """
         modifier to change the rule matching mode.

         In this mode, at least one rule must pass at least one match check
        """
        ...

# ========================================================


# ========================================================
class CartridgeBytes:
    """
 The cartridge is the container of the rules
 (recommendation to use the string version if possible. More information on `Rule`.)

 # Notes
 * Use a container for one object if possible. Imagine that one container is one specific error `NotFound`, `InvalidHeader`, `WrongCase`.
    """

    def __init__(self, id: int, message: str, root_rules: List[RuleBytes]) -> None:
        r"""
    Parameters
    ----------
    `id` : `int`
            Error code
    `message` : `str`
            Error message
    `root_rules` : `List[RuleBytes]`
            The rules to be added

    Constructor for `CartridgeBytes`

    # Notes

    By default, `all_rules_for_all_matches`. In this mode, all rules must be tested for all matches

    ## Fill in messages
     * Each cartridge supports filling the message with unwanted data, when specifying a message,
     you can specify a variable in the message in the format : **`{variable}`**.
     After specifying an identical group name in any rule along with the *`MustNotBeFound`* modifier
        """
        ...

    def any_r_for_any_m(self) -> CartridgeBytes:
        """
         modifier to change the rule matching mode.

         In this mode, at least one rule must pass at least one match check
        """
        ...
# ========================================================


# ========================================================
class TemplateValidator:
    """
    The structure for creating unique validators, load different `cartridges` to validate data.
    """

    def __init__(self, rules: List[Cartridge]) -> None:
        ...

    async def async_validate(self, text: str) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `str`
            Data for verification

        Returns
        ------
        `ValidationErrorIterator`
            This iterator stores an `error code` and a `message`
        `None`
            If there are no errors
        """
        ...

    def validate(self, text: str) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `str`
            Data for verification

        Returns
        ------
        `ValidationErrorIterator`
            This iterator stores an `error code` and a `message`
        `None`
            If there are no errors
        """
        ...
# ========================================================


# ========================================================
class TemplateValidatorBytes:
    """
    The structure for creating unique validators, load different `cartridges` to validate data
    (recommendation to use the string version if possible. More information on `Rule`.)
    """

    def __init__(self, rules: List[CartridgeBytes]) -> None:
        ...

    async def async_validate(self, text: bytes) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `bytes`
            Data for verification

        Returns
        ------
        `ValidationErrorIterator`
            This iterator stores an `error code` and a `message`
        `None`
            If there are no errors
        """
        ...

    def validate(self, text: bytes) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `bytes`
            Data for verification

        Returns
        ------
        `ValidationErrorIterator`
            This iterator stores an `error code` and a `message`
        `None`
            If there are no errors
        """
        ...
        ...
# ==========================================================


class ValidationError:
    pass

    def get_code(self) -> int:
        """
        Returns
        -------
        `int`
            Error code
        """
        ...

    def get_message(self) -> str:
        """
        Returns
        -------
        `str`
            Error message with data
        """
        ...
