Before we looked at modifiers that affect one `Rule`, but now we will study modifiers that affect all `subrules` within one `root rule`

- `all_rules_for_all_matches` (default mode)
- `all_rules_for_at_least_one_match (all_r_for_any_m)`
- `at_least_one_rule_for_all_matches (any_r_for_all_m)`
- `at_least_one_rule_for_at_least_one_match (any_r_for_any_m)`


### all_rules_for_all_matches 

In this mode, all rules must be tested for all matches

#### Operation scheme of the mode
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

### all_rules_for_at_least_one_match (all_r_for_any_m)

In this mode, all rules must pass the test for at least one match

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

### at_least_one_rule_for_all_matches (any_r_for_all_m)
---

In this mode, at least one rule must pass the test for all matches.

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

### at_least_one_rule_for_at_least_one_match (any_r_for_any_m)

In this mode, at least one rule must pass at least one match check

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