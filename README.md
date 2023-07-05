# Pystval 

<p align="center">
  <kbd>
    <img src="docs/images/logo_b.svg" alt="Logo" width="600"/>
  </kbd>
</p>

- [Pystval](#pystval)
  - [The Library](#the-library)
  - [Quick Look](#quick-look)
  - [Installation](#installation)
  - [Regex Syntax](#regex-syntax)
    - [](#)
  - [The system of rule modifiers](#the-system-of-rule-modifiers)
  - [Examples](#examples)
  - [License](#license)



## The Library

**Pystval** is a powerful `Rust` library for text validation and analysis. It allows you to create custom errors with modifiers and define sub-corrections for each rule. With Pystval, you can build versatile validators that handle any text and enforce specific requirements. Customize error behavior and apply sub-corrections based on specific cases or conditions. Pystval empowers you to create accurate and adaptable validators for various text processing needs.

## Quick Look

```python
 some code
```


## Installation

To install the latest version of Pystval, use the following command:
```bash
pip install pystval-version-platform.whl
```
**Supported Platforms**:

|                     | **Linux**                   | **Windows**               | **macOS**              |
| ------------------- | --------------------------- | ------------------------- | ---------------------- |
| System architecture | `x86_64-unknown-linux-gnu`  | `x86_64-pc-windows-msvc`  | `aarch64-apple-darwin` |
|                     | `aarch64-unknown-linux-gnu` | `aarch64-pc-windows-msvc` | `x86_64-apple-darwin`  |

## Regex Syntax

### 
- [**Default-Regex**](https://docs.rs/regex/latest/regex/#syntax)
- [**Fancy-Regex**](https://docs.rs/fancy-regex/latest/fancy_regex/#syntax)
OR
- [the same as those links, but with only a syntax table ](docs/syntax_regex/regex.md)

## The system of rule modifiers

## Examples
- [A simple example with fn `validate`](docs/examples/example_1.md)
- [A simple example with fn `async validate`](docs/examples/example_2.md)
- [An example with a large nesting of rules](docs/examples/example_3.md)
- [A simple `html` validator example](docs/examples/example_4.md)
- [A simple `css` validator example](docs/examples/example_5.md)
- [An example of a text search using complex criteria](docs/examples/example_6.md)


## License

Pystval is licensed under the [MIT License](LICENSE).