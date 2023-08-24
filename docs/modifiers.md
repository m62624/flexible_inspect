





## Matching text counter

Before we see the following modifiers, let's see how text match capture works when a rule is triggered, for example pattern `\d+`, for text `123 123 123 54 6 7 8`. We only get `123`, `6`, `7`, `8`. 

What happened now? 
For matches in the library we use [**IndexSet**](https://crates.io/crates/indexmap), this is necessary so as not to check once again all the rules of three matches `123`, but we always keep the number of identical matches. This way, we only keep unique values and keep a count of them so we always know how many times they are repeated.

### counter_is_equal `X`

Adding a match counter, where the condition is: 
> there must be exactly `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_is_equal(30)
```

### counter_more_than `X`

Adding a match counter, where the condition is: 
> there must be greater than or equal to `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_more_than(12)
```

### counter_less_than `X`

Adding a match counter, where the condition is: 
> there must be less than or equal to `x` matches

```python
 Rule(r"\[\d+\]", MatchRequirement.MustBeFound).counter_less_than(1000)
```
