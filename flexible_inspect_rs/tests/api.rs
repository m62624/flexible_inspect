use flexible_inspect_rs::prelude::*;

mod api_rules {
    use super::*;

    #[test]
    fn check() {
        let cartridge = Cartridge::new(
            1,
            "Incorrect command found `{main_capture}`",
            [Rule::new(
                r"sudo rm -rf /?",
                MatchRequirement::MustNotBeFound,
            )],
        );

        let bash_script = r###"
        #!/bin/bash
        if [ "$1" = "--help" ]; then
            echo "This is a sample script with --help option."
            echo "Usage: ./myscript.sh [--help]"
            echo "If you provide the --help option, it will execute the dangerous command 'sudo rm -rf /'."
            echo "Use with caution!"
        else
            echo "Welcome to the safe area!"
        fi        
        "###;

        let validator_for_linux_system = TemplateValidator::new([cartridge]);
        if let Err(errors) = validator_for_linux_system.validate(bash_script) {
            for error in errors {
                println!("{}", error);
            }
        }

    }
}
