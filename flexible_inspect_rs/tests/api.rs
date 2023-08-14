use flexible_inspect_rs::prelude::*;

mod api_rules {
    use super::*;

    #[async_std::test]
    async fn check_bash() {
        let bash_script = r###"
#!/bin/bash

if [ $# -lt 1 ]; then
    echo "Usage: $0 <number> <word> <username>"
    exit 1
fi

if [ "$1" -eq "$1" ] 2>/dev/null; then
    for ((i=0; i<$1; i++)); do
        echo "Short message for user $3"
    done
fi

if [ -n "$2" ]; then
    echo "Word: $2"
fi

if [ -n "$3" ]; then
    echo "Warning: Using sudo_su as $3 can be dangerous"
    sudo rm 1234_important_file.txt
fi

if [[ -n "$4" && "$4" =~ ^[0-9]{8,}$ ]]; then
    echo "Password set: $4"
fi        
"###;

        let forbidden_sudo = Cartridge::new(
            0,
            "Scripts with increased access are forbidden : {SUDO_OR_SU}",
            [
                Rule::new(r"(?P<SUDO_OR_SU>sudo .+)", MatchRequirement::MustNotBeFound),
                Rule::new(r"(?P<SUDO_OR_SU>su .+)", MatchRequirement::MustNotBeFound),
            ],
        );
        let incorrect_header = Cartridge::new(
            1,
            "Inccorect type file",
            [Rule::new(
                r"^\n?#!/bin/bash\s+?\n?",
                MatchRequirement::MustBeFound,
            )],
        );

        let incorrect_body = Cartridge::new(
            2,
            "Inccorect fragment code",
            [
                Rule::new(r"(?s).+", MatchRequirement::MustBeFound).extend([Rule::new(
                    r"if.+then",
                    MatchRequirement::MustBeFound,
                )
                .counter_is_equal(5)]),
            ],
        );

        let validator_bash =
            TemplateValidator::new(vec![forbidden_sudo, incorrect_header, incorrect_body]);
        if let Err(errors) = validator_bash.async_validate(bash_script).await {
            for error in errors {
                println!("{}", error);
            }
        }
    }
}
