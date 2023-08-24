# Modifier(methods)

## MatchRequirement

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

## Extend Rule

{% include "./extend.md" %}

## Matching mode

{% include "./matching_mode.md" %}