## How to debug the code ?

How to understand what the expression caught, what rules worked ? for which rule was the modifier used ? 

The library implements logging modes via an environment variable.

=== "Rust"

    ``` bash
    FLEX_VALIDATOR_LOG=INFO cargo run
    ```
    or
    ``` rust
    use std::env;
    // ERROR, INFO, DEBUG, TRACE
    env::set_var("FLEX_VALIDATOR_LOG", "INFO");
    // some code
    ```

    !!! info 
        If you call the `init_logger_with_offset` function to shift the time in the logs, the declaration of the environment variable through the code must be before the `init_logger_with_offset` functions are called. 

=== "JavaScript/TypeScript"

    Initialize logging before you start using library items
  
    !!! info
        Unlike other languages, reading environment variables is not supported in this library. Therefore, a call to `init_logger()` is required to enable logging.

    ``` js
     init_logger(LogLevel.INFO);
    ```

=== "Python"

    ``` bash
    FLEX_VALIDATOR_LOG=INFO file.py
    ```
    or
    ``` python
    import os
    # ERROR, INFO, DEBUG, TRACE
    os.environ["FLEX_VALIDATOR_LOG"] = "INFO"
    # some code
    ```
    
    !!! info 
        If you call the `init_logger_with_offset` function to shift the time in the logs, the declaration of the environment variable through the code must be before the `init_logger_with_offset` functions are called.

### Levels of logging support:
 * `ERROR` - only errors are displayed (always displayed in any mode)
 > Includes all messages that did not pass the rules condition test
 * `INFO` - errors and information are displayed
 > Includes all messages that did not pass the rules condition test and information about the rules that were caught
 * `DEBUG` - errors, information and debug information are displayed
 > Includes all messages that did not pass the rules condition test, information about the rules that were caught and debug information about debug what modifiers are applied
 * `TRACE` - errors, information, debug information and trace information are displayed
 > Includes all messages that did not pass the rules condition test, information about the rules that were caught, debug information about debug what modifiers are applied and trace information about what type of regular expression is used, and whether it is included in the [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html).