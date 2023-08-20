# Changelog

## [2.0.0] - 2023.08.19
### Fixed
- Fixes for mode switching in nested rule matches.
### Changed
- Enhancements to the behavior of all modes.
- Utilized IndexSet instead of `Vec<Rule or RuleBytes>` for better `contains` method performance and duplicate rule removal.
- Complete restructuring and renaming of the library: `pystval` has been changed to `flexible_inspect_*`, enabling multi-language support.
- Now supports (all versions use the **rust** version of the library as a base):
  - `Rust`
  - `Python`
  - `JavaScript` (including `TypeScript` using [WASM](https://webassembly.org) format).
- Added serialization and deserialization support for Rust versions (use feature = [features]).
- Changed the return type of the validator to `Result<(), Vec<ValidationError>>` for both `validate` and `async_validate` functions. Both functions now return a collection of errors that did not pass validation.

### Removed
- Removed auto-generation of regular expressions within `Rule` and `RuleBytes` structures. This feature may return in the future.

## [1.1.0] - 2023.07.19

### Added
- New static method `auto_generate` in `Rule` class. This method allows users to generate regular expressions based on given text input. This is especially useful for quick pattern prototyping. It takes a `MatchRequirement` and a list of strings as input and returns a `Rule` instance with a generated regular expression.

### Fixed
- Issue with the `matching mod`. The problem with `all_rules_for_all_matches` not allowing the upper stack to transition to other modes has been resolved.

### Changed
- For user convenience, all documentation from the `README` file has been transferred to Read the Docs. Now, you can find comprehensive, neatly organized, and searchable project documentation at our [Read the Docs](https://readthedocs.org/projects/pystval/) page.


## [1.0.0] - 2023.07.10

### Added
- Combination of different rules with modifiers, with results of these rules automatically passed to subexpressions.
- Installation via PyPi, Github packages

### Features
- `Cartridge`: A structure that contains a class for creating an error, class rules by which the check is done, and a message with a possible valid value.
- `Rule`: A structure for storing a regular expression with modifiers.
- `Root rule`: All rules that are in the first step of the cartridge.
- `Subrule`: The rules that are below the first step of the cartridge, as well as all rules created inside the extend method.
- `Simple regex`: Category rules based on the `regex` library.
- `Complex regex`: Category rules based on the `fancy-regex` library.
- `Modifier`: Additional logic changes for Rule.
- Matching mode: Modifiers that affect all subrules within one root rule.
- Matching text counter: Keeps the number of identical matches.
- Filling in the error message variables: Allows to specify the same name as the name of the capture group in a regular expression.
- Logging system for tracking validation processes and errors.

Please note that this is a brief summary of the changes and features introduced in version `1.0.0`. For a detailed understanding of the features and how to use them, please refer to the `README` file or the official documentation.


## [0.4.0] - 2023.05.15
### Added
 - Working *API*, most likely on the basis of versions `0.4.0` will come `1.0.0` version
 - A documented *API* that will be used as tips for the *IDE*