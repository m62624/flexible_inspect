use flexible_inspect_rs::prelude::*;
const BASH_SCRIPT: &str = r###"
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

#[async_std::test]
async fn test_validate_0() {
    let error_bash_file = Cartridge::new(
        0,
        "Scripts with increased access are forbidden : {SUDO_OR_SU}",
        [
            Rule::new(r"(?P<SUDO_OR_SU>sudo .+)", MatchRequirement::MustNotBeFound),
            Rule::new(r"(?P<SUDO_OR_SU>su .+)", MatchRequirement::MustNotBeFound),
            Rule::new(r".+", MatchRequirement::MustBeFound)
                .extend([
                    Rule::new(".+", MatchRequirement::MustBeFound)
                        .extend([Rule::new(r"aboba", MatchRequirement::MustBeFound)]),
                    Rule::new(r"\s", MatchRequirement::MustBeFound),
                    Rule::new(r"(?P<USERNAME>\for .+\n)", MatchRequirement::MustBeFound),
                ])
                .any_r_for_all_m(),
        ],
    )
    .any_r_for_any_m();
    let validator_bash = TemplateValidator::new(vec![error_bash_file]);
    assert!(validator_bash.async_validate(BASH_SCRIPT).await.is_ok());
}

#[async_std::test]
async fn test_valdiate_1() {
    let error_bash_file = Cartridge::new(
        0,
        "Scripts with increased access are forbidden : {SUDO_OR_SU}",
        [
            Rule::new(r"(?P<SUDO_OR_SU>sudo .+)", MatchRequirement::MustNotBeFound),
            Rule::new(r"(?P<SUDO_OR_SU>su .+)", MatchRequirement::MustNotBeFound),
        ],
    );
    let validator_bash = TemplateValidator::new(vec![error_bash_file]);
    assert!(validator_bash.async_validate(BASH_SCRIPT).await.is_err());
}
