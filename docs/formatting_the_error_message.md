# Filling in the error message variables

When we put the same names in error messages, and in a rule with the `MustNotBeFound` modifier, we fill the message with the data we got from the rule, and the thing to remember here is that no matter how many groups you create in one regular expression, we save the data from the entire regular expression 

=== "Rust"

    ```rust
    // text = "123123sd"
    Rule::new(r"(?P<NUMBER_3>\d{3})(?P=NUMBER_3)\w+", MatchRequirement::MustBeFound)
    ``` 
    
    ```rust
    // Output (log)
    Some(Captures({
        0: Some("123123sds"),
        1: Some("123"),
    })),
    ```
=== "JavaScript/TypeScript"

    ``` js
    // text = "123123sd"
    Rule(String.raw`(?P<NUMBER_3>\d{3})(?P=NUMBER_3)\w+`, MatchRequirement.MustBeFound)
    ``` 
        
    ```rust
    // Output (log)
    Some(Captures({
        0: Some("123123sds"),
        1: Some("123"),
    })),
    ```

=== "Python"

    ```python
    # text = "123123sd"
    Rule(r"(?P<NUMBER_3>\d{3})(?P=NUMBER_3)\w+", MatchRequirement.MustBeFound)
    ``` 

    ```rust
    // Output (log)
    Some(Captures({
        0: Some("123123sds"),
        1: Some("123"),
    })),
    ```

we skip the match from the `NUMBER_3` (**1**) subgroup, we keep the overall result from the expression (**0**).

---
When you specify the `message` in Cartridge
 

=== "Rust"

    ```rust
    let cartrdige_0 = Cartridge::new(
            0,
            "error message with value {INFO}",
            [...],
        );
    ```

=== "JavaScript/TypeScript"

    ``` js
    let cartrdige_0 = new Cartridge(
            0,
            "error message with value {INFO}",
            [...],
        );
    ```

=== "Python"

    ```python
    cartrdige_0 = Cartridge(
            0,
            "error message with value {INFO}",
            [...],
        )
    ```

you can specify the same name as the name of the capture group in a regular expression

=== "Rust"

    ```rust
    Rule::new(r"(?<INFO>\d+)", MatchRequirement::MustNotBeFound)
    ```

=== "JavaScript/TypeScript"

    ``` js
    Rule(String.raw`(?<INFO>\d+)`, MatchRequirement.MustNotBeFound)
    ```

=== "Python"

    ```python
     Rule(r"(?<INFO>\d+)", MatchRequirement.MustNotBeFound)
    ```

If you want to output only one variable, you can use the reserved name without assigning a group name to the regular mode 
`main_capture`.

=== "Rust"

    ```rust
    let cartrdige_0 = Cartridge::new(
            0,
            "error message with value {main_capture}",
            [
                Rule::new(r"(?<main_capture>\d+)", MatchRequirement::MustNotBeFound),
            ],
        );
    ```

=== "JavaScript/TypeScript"

    ``` js
    let cartrdige_0 = new Cartridge(
            0,
            "error message with value {main_capture}",
            [
                Rule(String.raw`(?<main_capture>\d+)`, MatchRequirement.MustNotBeFound),
            ],
        );
    ```

=== "Python"

    ```python
    cartrdige_0 = Cartridge(
            0,
            "error message with value {main_capture}",
            [
                Rule(r"(?<main_capture>\d+)", MatchRequirement.MustNotBeFound),
            ],
        )
    ```