Now we can start writing the code. Let's start with importing

=== "Rust"

    ``` rust
    use flexible_inspect_rs::prelude::*;
    ```

=== "JavaScript/TypeScript"

    If you are using the **node** version

    ``` js
    import {
      Rule,
      MatchRequirement,
      Cartridge,
      TemplateValidator,
      init_logger,
      LogLevel,
    } from "flexible_inspect_js_node";
    ```

    Or you use the **web** version to work directly in the **browser** or using **webpack**. You must load the `wasm` file before using the library, using the `init()` async function.

    ??? info "if you use webpack "
      
        Don't forget to add the experimental option to the webpack config. 

        ``` js
        module.exports = {
          experiments: {
            asyncWebAssembly: true,
          },
        }
        ```

    ``` js
    //--------|
    //        |
    //        v
    import init, {
      Rule,
      MatchRequirement,
      Cartridge,
      TemplateValidator,
      init_logger,
      LogLevel,
    } from "@m62624/flexible_inspect_js_web";

    init().then(
      () => {
        console.log("WASM module loaded");
        // some code that uses the WASM module
      },
      (err) => {
        console.log("Error loading WASM module:", err);
      }
    );
    ```

=== "Python"

    ``` python
    from flexible_inspect_py import Cartridge, MatchRequirement, TemplateValidator, Rule
    ```

Next, let's look at the text for which validation will take place. It's just json-like mixed with plain text. Let's just say this is just a report on some kind of system test.

``` json
    { 
        v1: 1,
        SYSTEM DATA FOR TESTS
        { "report": {
            #BAD_TOKEN_MESSAGE-123312-🎃#
          { "title": "Test Results",
          { "date": "2023-08-20",
          { "tests": [ ---------- MARK @@21 [secret-ket 111-222-333-GG]
            {
              "title": "Performance Testing",
              STABLE AND UNCHANGED DATA = 1234567890 [
                "result": "successful", 
                { "details": (
                    @@@@ MARK @@21 [secret-ket 111-222-333-GG]
                    { "start_time": "9:56",
                    { "end_time": "12:00",
                    { "past_iterations": 1000,
                    { "average_time_iteration": "0.03 sec"
                )
              ] #BAD_TOKEN-MESSAGE#
              "result": "successful", 
              { "details": { #BAD_TOKEN_MESSAGE--{}{][][123#
                { "start_time": "10:00",
                "end_time": "10:30",
                "past_iterations": 1000,
                "average_time_iteration": "0.03 sec"
              } [Convert data to bytes] === === RESULT: [0x12, 0x34, 0x56, 0x78]
              | | | | | |

              | | | | | |
            },
            {
              }, { "title": "Stability Testing",
              { "result": "not_successful",
              }, "details": {
                "errors": 5, #BAD_TOKEN_MESSAGE-OQLWLQLW#
                "important_warning": 2,
                { "end_time": "12:45"
              }
            },
      END OF SYSTEM DATA FOR TESTS
```

We'll validate for two errors

!!! abstract "Error 1 & Error 2"
    - Check incorrect *tokens*, and get the first incorrect *token*.
    - Check in the `"Performance Testing"` body that the test was completed no later than **11:00**, (check the time if the test was successful)

To do this, we'll create two cartridges

=== "Rust"

    ``` rust
    // Cartridge for checking incorrect tokens received
    let found_broken_token = Cartridge::new(
        -10, // error code
        "Found a broken token {bd_tkn}",
        [Rule::new(
            "(?<bd_tkn>#BAD.TOKEN.MESSAGE.+?#)",
            MatchRequirement::MustNotBeFound,
        )],
    );
    /*
    check under `Performance Testing` that the end time must be earlier than 11 o'clock,
    check the time only if the result is successful
     */
    let long_performance_testing = Cartridge::new(
        1100, // error code
        "The test did not pass within the given time (before 11:00 hours)",
        [
            // get the body of Performance Testing
            Rule::new(
                r#"(?ms)"title":\s?"Performance Testing",\s.*\)"#,
                MatchRequirement::MustBeFound,
            )
            // Get the result from the root to this rule, we got the Performance Testing body,
            //  now check the result of the test
            .extend([Rule::new(
                r#"(?ms)"result":\s?"successful".+\)"#,
                MatchRequirement::MustNotBeFound,
            )
            // the time must be no later than 11:00
            .extend([Rule::new(
                r#""end_time": "(?:(?:0[0-9]|1[0-1]):[0-5][0-9])""#,
                MatchRequirement::MustBeFound,
            )])]),
        ],
    );
    ```

=== "JavaScript/TypeScript"

    !!! warning "`finish_build()`"

        One thing to remember in `JavaScript/TypeScript` is that the classes:
        `Cartridge`, `CartridgeBytes`, `Rule`, `RuleBytes` before sending them to any methods that accept these structures, you must specify `finish_build()`, this method prepares the structure to work in `Rust`.

        That is, you can initialize your variable, use various modifiers and at the end specify `finish_build()`, after that the structure cannot use its methods.

    ``` js
    // Cartridge for checking incorrect tokens received
    let found_broken_token = new Cartridge(
      -10, // error code
      "Found a broken token {bd_tkn}",
      [
        new Rule(
          String.raw`(?<bd_tkn>#BAD.TOKEN.MESSAGE.+?#)`,
          MatchRequirement.MustNotBeFound
        ).finish_build(),
      ]
    );
    /*
      check under `Performance Testing` that the end time must be earlier than 11 o'clock,
      check the time only if the result is successful
    */
    let long_performance_testing = new Cartridge(
      1100, // error code
      "The test did not pass within the given time (before 11:00 hours)",
      [
        // get the body of Performance Testing
        new Rule(
          String.raw`(?ms)"title":\s?"Performance Testing",\s.*\)`,
          MatchRequirement.MustBeFound
        )
          // Get the result from the root to this rule, we got the Performance Testing body,
          //  now check the result of the test
          .extend([
            new Rule(
              String.raw`(?ms)"result":\s?"successful".+\)`,
              MatchRequirement.MustNotBeFound
            )
              // the time must be no later than 11:00
              .extend([
                new Rule(
                  String.raw`"end_time": "(?: (?: 0[0 - 9] | 1[0 - 1]): [0 - 5][0 - 9])"`,
                  MatchRequirement.MustBeFound
                ).finish_build(),
              ])
              .finish_build(),
          ])
          .finish_build(),
      ]
    );
    ```

=== "Python"

    ``` python
    # Cartridge for checking incorrect tokens received
    found_broken_token = Cartridge(-10, "Found a broken token {bd_tkn}", [
        Rule(
        r"(?<bd_tkn>#BAD.TOKEN.MESSAGE.+?#)",
        MatchRequirement.MustNotBeFound,
        )]
    )

    # check under `Performance Testing` that the end time must be earlier than 11 o'clock,
    # check the time only if the result is successful
    long_performance_testing = Cartridge(
    1100,
    "The test did not pass within the given time (before 11:00 hours)",
    [
        # get the body of Performance Testing
        Rule(
            r'(?ms)"title":\s?"Performance Testing",\s.*\)',
            MatchRequirement.MustBeFound,
        )
        # Get the result from the root to this rule, we got the Performance Testing body,
        #  now check the result of the test
        .extend(
            [
                Rule(
                    r'(?ms)"result":\s?"successful".+\)',
                    MatchRequirement.MustNotBeFound,
                )
                # the time must be no later than 11:00
                .extend([Rule(
                        r'"end_time": "(?:(?:0[0-9]|1[0-1]):[0-5][0-9])"',
                        MatchRequirement.MustBeFound,
                        )
                ])
            ]
        ),
    ],
    )
    ```
So, we have created two cartridges, within which we have defined rules that store regular expressions with modifiers. 
Each cartridge has a default validation mode `all root rules must be successfully validated`, the same applies to nested rules. You may notice in the second cartridge we used `extend` for the rule, when you use this modifier, you create a nested rule, the nested rules get the results from the root rule and start checking them.
Here is an example of how the standard mode of nested rules works : `all root rules must be successfully validated`

``` bash
     #=======================================
     text = "txt [123] txt [456] txt [789]"
     #=======================================
     CustomError
     |
     |__ Rule "\[[^\[\]]+\]" (MustBeFound) 
          |   [123], [456], [789] # this is the result of the root rule
          |___ Subrule ".+" (MustBeFound) ---> [123] -> [456] -> [789] -- TRUE
          |                                      |       |        |
          |___ Subrule "\[\d+\]" (MustBeFound) __|_______|________|
```
Let's now add a logging mode before we initialize our cartridges to see how the rules worked out

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

    Don't forget to add to the import `init_logger`, `LogLevel`. 
  
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

After installing the logs, now let's run validation, load our validator with cartridges, and then, if something does not pass the check, we will get an iterator, which stores the object with an error code and a error message

=== "Rust"

    ``` rust
    let validator_for_pseudo_format = TemplateValidator::new([found_broken_token, long_performance_testing]);
    if let Err(errors) = validator_for_pseudo_format.validate(text) {
        for err in errors {
            println!("{}", err);
        }
    }
    ```

=== "JavaScript"

    ``` js
    let validator_for_pseudo_json = new TemplateValidator([
        found_broken_token,
        long_performance_testing,
    ]);
  
  
    let result = validator_for_pseudo_json.validate(text);
      if (result !== undefined) {
        result.for_each_1((error_code, error_message) => {
          console.log(error_code, error_message);
        });
      }
    } 
    ```

=== "TypeScript"

    ``` ts

    let validator_for_pseudo_json = new TemplateValidator([
      found_broken_token,
      long_performance_testing,
    ]);


    let result = validator_for_pseudo_json.validate(text);
    if (result !== undefined) {
      result.for_each_1((error_code: number, error_message: string) => {
        console.log(error_code, error_message);
      });
    }
    },
    ```

=== "Python"

    ``` python
    
    ```