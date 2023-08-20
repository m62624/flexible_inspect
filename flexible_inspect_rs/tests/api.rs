use flexible_inspect_rs::prelude::*;
#[test]
fn main_rust() {
    // This is text with a pseudo format for validations
    let text = r###"
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
            {
              }, { "title": "Compatibility Testing",
              "result": "successful" #BAD_TOKEN_MESSAGE-ppp12003#
              }, "details": {
                { "supported_platforms": ["Windows", "Linux", "macOS"],
                }, "end_time": "14:20"
              }
            }
          ] #BAD_TOKEN_MESSAGE-12031293193# ==== MARK @@20 [------]
        }
      }
      END OF SYSTEM DATA FOR TESTS
    "###;
    // Cartridge for checking incorrect tokens received
    let found_broken_token = Cartridge::new(
        0000,
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
        0001,
        "Found a broken token {bd_tkn}",
        [
            Rule::new(
                r"(?+)Performance Testing.+",
                MatchRequirement::MustBeFound,
            ), // .extend([Rule::new(
               //     r###""result":\s?"successful""###,
               //     MatchRequirement::MustBeFound,
               // )])
        ],
    );

    let validate_for_pseudo_format =
        TemplateValidator::new([found_broken_token, long_performance_testing]);
    if let Err(errors) = validate_for_pseudo_format.validate(text) {
        for err in errors {
            println!("{}", err);
        }
    }
}
