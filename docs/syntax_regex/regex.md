- [Links](#links)
  - [Default Regex](#default-regex)
  - [Fancy Regex](#fancy-regex)


# Links
> Text taken from these links (full details) : 
> - [ (`regex`) docs.rs](https://docs.rs/regex/latest/regex/)
> - [ (`fancy-regex`) docs.rs](https://docs.rs/fancy-regex/latest/fancy_regex/)

---

## Default Regex

**Matching one character**:
|             |                                                                 |
| ----------- | --------------------------------------------------------------- |
| `.`         | any character except new line (includes new line with s flag)   |
| `\d`        | digit (`\p{Nd}`)                                                |
| `\D`        | not digit                                                       |
| `\pX`       | Unicode character class identified by a one-letter name         |
| `\p{Greek}` | Unicode character class (general category or script)            |
| `\PX`       | Negated Unicode character class identified by a one-letter name |
| `\P{Greek}` | negated Unicode character class (general category or script)    |

**Character classes**:
|                |                                                                         |
| -------------- | ----------------------------------------------------------------------- |
| `[xyz]`        | A character class matching either x, y or z (union).                    |
| `[^xyz]`       | A character class matching any character except x, y and z.             |
| `[a-z]`        | A character class matching any character in range a-z.                  |
| `[[:alpha:]]`  | ASCII character class ([A-Za-z])                                        |
| `[[:^alpha:]]` | Negated ASCII character class ([^A-Za-z])                               |
| `[x[^xyz]]`    | Nested/grouping character class (matching any character except y and z) |
| `[a-y&&xyz]`   | Intersection (matching x or y)                                          |
| `[0-9&&[^4]]`  | Subtraction using intersection and negation (matching 0-9 except 4)     |
| `[0-9--4]`     | Direct subtraction (matching 0-9 except 4)                              |
| `[a-g~~b-h]`   | Symmetric difference (matching `a` and `h` only)                        |
| `[\[\]]`       | Escaping in character classes (matching [ or ])                         |

**Composites**:
|      |                                 |
| ---- | ------------------------------- |
| `xy` | concatenation (x followed by y) |
| `x   | y`                              | alternation (x or y, prefer x) |

**Repetitions**:
|           |                                              |
| --------- | -------------------------------------------- |
| `x*`      | zero or more of x (greedy)                   |
| `x+`      | one or more of x (greedy)                    |
| `x?`      | zero or one of x (greedy)                    |
| `x*?`     | zero or more of x (ungreedy/lazy)            |
| `x+?`     | one or more of x (ungreedy/lazy)             |
| `x??`     | zero or one of x (ungreedy/lazy)             |
| `x{n,m}`  | at least n x and at most m x (greedy)        |
| `x{n,}`   | at least n x (greedy)                        |
| `x{n}`    | exactly n x                                  |
| `x{n,m}?` | at least n x and at most m x (ungreedy/lazy) |
| `x{n,}?`  | at least n x (ungreedy/lazy)                 |
| `x{n}?`   | exactly n x                                  |

**Empty matches**:
|      |                                                                             |
| ---- | --------------------------------------------------------------------------- |
| `^`  | the beginning of text (or start-of-line with multi-line mode)               |
| `$`  | the end of text (or end-of-line with multi-line mode)                       |
| `\A` | only the beginning of text (even with multi-line mode enabled)              |
| `\z` | only the end of text (even with multi-line mode enabled)                    |
| `\b` | a Unicode word boundary (`\w` on one side and `\W`, `\A`, or `\z` on other) |
| `\B` | not a Unicode word boundary                                                 |

**Grouping and flags**
|                 |                                                                   |
| --------------- | ----------------------------------------------------------------- |
| `(exp)`         | numbered capture group (indexed by opening parenthesis)           |
| `(?P<name>exp)` | named (also numbered) capture group (names must be alpha-numeric) |
| `(?<name>exp)`  | named (also numbered) capture group (names must be alpha-numeric) |
| `(?:exp)`       | non-capturing group                                               |
| `(?flags)`      | set flags within current group                                    |
| `(?flags:exp)`  | set flags for exp (non-capturing)                                 |

|     |                                                                              |
| --- | ---------------------------------------------------------------------------- |
| `i` | case-insensitive: letters match both upper and lower case                    |
| `m` | multi-line mode: `^` and `$` match begin/end of line                         |
| `s` | allow `.` to match `\n`                                                      |
| `U` | swap the meaning of `x*` and `x*?`                                           |
| `u` | Unicode support (enabled by default)                                         |
| `x` | verbose mode, ignores whitespace and allow line comments (starting with `#`) |

**Perl character classes (Unicode friendly)**
|      |                                                                                    |
| ---- | ---------------------------------------------------------------------------------- |
| `\d` | digit (`\p{Nd}`)                                                                   |
| `\D` | not digit                                                                          |
| `\s` | whitespace (`\p{White_Space}`)                                                     |
| `\S` | not whitespace                                                                     |
| `\w` | word character (`\p{Alphabetic}` + `\p{M}` + `\d` + `\p{Pc}` + `\p{Join_Control}`) |
| `\W` | not word character                                                                 |

## Fancy Regex

**Escapes:**
|      |                                                   |
| ---- | ------------------------------------------------- |
| `\h` | hex digit (`[0-9A-Fa-f]`)                         |
| `\H` | not hex digit (`[^0-9A-Fa-f]`)                    |
| `\e` | escape control character (`\x1B`)                 |
| `\K` | keep text matched so far out of the overall match |
| `\G` | anchor to where the previous match ended          |

**Backreferences**:
|      |                                                             |
| ---- | ----------------------------------------------------------- |
| `\1` | match the exact string that the first capture group matched |
| `\2` | backref to the second capture group, etc                    |

**Named capture groups**:
|                 |                                                                  |
| --------------- | ---------------------------------------------------------------- |
| `(?<name>exp)`  | match exp, creating capture group named name                     |
| `\k<name>`      | match the exact string that the capture group named name matched |
| `(?P<name>exp)` | same as `(?<name>exp)` for compatibility with `Python`, etc.     |
| `(?P=name)`     | same as `\k<name>` for compatibility with `Python`, etc.         |

**Look-around assertions for matching without changing the current position**:

|            |                                                                          |
| ---------- | ------------------------------------------------------------------------ |
| `(?=exp)`  | look-ahead, succeeds if exp matches to the right of the current position |
| `(?!exp)`  | negative look-ahead, succeeds if exp doesn’t match to the right          |
| `(?<=exp)` | look-behind, succeeds if exp matches to the left of the current position |
| `(?<!exp)` | negative look-behind, succeeds if exp doesn’t match to the left          |