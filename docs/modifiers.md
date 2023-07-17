# Modifier
Modifiers are additional logic changes for `Rule`. Modifiers are an important element of the `rules`.  When you create a `Rule` there are always at least two modifiers created. 

The first one is the category of the pattern. The `Simple regex` or `Complex regex` is a hidden modifier, it cannot be called for the `Rule`, but it is defined based on the pattern. When you create a regular expression without leading and trailing checks, it will be put into the category `Simple Rule`, and will be used in the `RegexSet` for each text. If you create a regular expression with leading and trailing checks, the rule goes to the end of the queue. So we go through an easy regular expression at the beginning and a hard one at the end.

## MatchRequirement

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
### Different situations 

As you may have noticed, there is a difference between these two options: 

| MatchRequirement   | Match found | does this rule have any subrules ? | Result                                   |
| ------------------ | ----------- | ---------------------------------- | ---------------------------------------- |
| **MustNotBeFound** | **Yes**     | **Yes**                            | **Continue processing subrules**         |
| MustNotBeFound     | Yes         | No                                 | Error (match should not have been found) |

This is done so that if you should not find this, but you do find it, you can create a subrules for additional checks with modifiers. If nothing is found, the subcorrections will simply be skipped.

## Extend rule

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

## Matching mode
Before we looked at modifiers that affect one `Rule`, but now we will study modifiers that affect all `subrules` within one `root rule`

- `all_rules_for_all_matches`
- `all_rules_for_at_least_one_match`
- `at_least_one_rule_for_all_matches`
- `at_least_one_rule_for_at_least_one_match`


### `all_rules_for_all_matches` 


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

### `all_rules_for_at_least_one_match`

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

### `at_least_one_rule_for_all_matches` 
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

### `at_least_one_rule_for_at_least_one_match`

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

## Matching text counter

Before we see the following modifiers, let's see how text match capture works when a rule is triggered, for example pattern `\d+`, for text `123 123 123 54 6 7 8`. We only get `123`, `6`, `7`, `8`, what happened now?, for matches in the library we use `HashSet<&'s str>`, this is necessary so as not to check once again all the rules of three matches `123`, BUT we always keep the number of identical matches. By doing this, we save only unique values and keep their counter so that we always know how many times they are repeated

### counter_is_equal `X`

Adding a match counter, where the condition is: 
there must be exactly `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)
```

### counter_more_than `X`

Adding a match counter, where the condition is: 
there must be greater than or equal to `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_more_than(12)
```

### counter_less_than `X`

Adding a match counter, where the condition is: 
there must be less than or equal to `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_less_than(1000)
```
