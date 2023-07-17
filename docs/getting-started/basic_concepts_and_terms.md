
Before explaining how the library works, let's understand the terminology.
We have such concepts as : 

- `Cartridge`
- `Rule`
- `Simple regex`
- `Complex regex`
- `Modifier`


### Cartridge
A `cartridge` is a structure that contains a class for creating an error, class rules by which the check is done, and a message with a possible valid value. For `Python`, the `cartridge` is your error class inherited from `PystvalException`. Based on different cartridges you can create validators with different validation conditions.

### Rule
The `rule` is a structure for storing a regular expression with modifiers. Structure is the basic minimum unit of the validation logic

#### Root rule
The `root rule` - all rules that are in the first step of the `cardridge`, and is also the root in relation to the `subrule`

#### Subrule
The `subrule` is the rules that are below the first step of the cartridge, as well as all rules created inside the `extend` method.
> But then again, even if `subrule` (**A**) is created within extend, for all `subrule` (**B** of **A**), `subrule` A itself will be the `root` for them

### Syntax regex
***Since all calculations using regex take place in `Rust`, it is necessary to follow the format `rust regex`.***

More information on syntax :

- [Default-Regex](https://docs.rs/regex/latest/regex/#syntax)
- [Fancy-Regex](https://docs.rs/fancy-regex/latest/fancy_regex/#syntax)
- [the same as those links, but with only a syntax table ](../syntax_regex/regex.md)

#### Simple regex
Category rules based on the [**regex**](https://docs.rs/regex/latest/regex/) library.
This package is optimized for fast execution and low memory consumption. It uses efficient algorithms and data structures to minimize memory usage while processing regular expressions.

The `regex` of rust provides us with [`RegexSet`](https://docs.rs/regex/latest/regex/struct.RegexSet.html), a data structure which allows us to match a string with a set of regular expressions at the same time. This can be useful when you have a lot of regular expressions and want to quickly determine which ones match a particular string.

The main advantage of using RegexSet is that it can be much faster than applying each regular expression to a string in sequence, especially if you have a large number of regular expressions.

> **The text is taken from the official `rust regex` documentation**\
> For example, consider regular expressions to match email addresses and domains: [a-z]+@[a-z]+\.(com|org|net) and [a-z]+\.(com|org|net). If a regex set is constructed from those regexes, then searching the haystack foo@example.com will report both regexes as matching. Of course, one could accomplish this by compiling each regex on its own and doing two searches over the haystack. The key advantage of using a regex set is that it will report the matching regexes using a single pass through the haystack. If one has hundreds or thousands of regexes to match repeatedly (like a URL router for a complex web application or a user agent matcher), then a regex set can realize huge performance gains. 

#### Complex regex

Category rules based on the [**fancy-regex**](https://docs.rs/fancy-regex/latest/fancy_regex/) library. This package supports more complex regular expression functions that may consume more memory. For example, backlinks and forward looking statements may require additional memory to store intermediate results and processing states.

It is important to note that the exact amount of memory consumed will depend on the specific regular expressions and data you are working with.