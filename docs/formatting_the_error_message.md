# Filling in the error message variables

When we put the same names in error messages, and in a rule with the `MustNotBeFound` modifier, we fill the message with the data we got from the rule, and the thing to remember here is that no matter how many groups you create in one regular expression, we save the data from the entire regular expression 

```python
text = "123123sd"
Rule(r"(?P<NUMBER_3>\d{3})(?P=NUMBER_3)\w+", MatchRequirement.MustBeFound)
``` 
```rust
// Output
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