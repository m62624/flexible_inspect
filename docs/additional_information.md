## Realizing ownership in other languages

While in Rust [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#what-is-ownership) works out of the box, using `self` to absorb and after applying modifiers, we get a modified value, in implementations of other languages, when we absorbed a value, we don't get a proper warning from the compiler that the value is already absorbed (since all implementations of the library in other languages are based on `Rust`). That's why we made an implementation on [borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) after which [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#what-is-ownership) occurs, without expensive cloning and checking [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#what-is-ownership).

Here's what the source code for the rust modifier `counter_more_than(x)` looks like

```rust
    // -----------------------|
    //                        |
    //                        v
    fn counter_more_than(mut self, count: usize) -> Self::RuleType {
        self.0.general_modifiers.counter = Some(Counter::MoreThan(count));
        debug!(
            "the `{}` modifier is applied to Rule ({}, {})",
            format!("counter_more_than ({})", count).bright_yellow(),
            self.get_str().yellow(),
            format!("{:#?}", self.get_requirement()).yellow()
        );
        self
    }
```

As we can see we don't use cloning here, we just take the body of the structure, change the value and revert it. Purely technical, the same can be done for `wasm` versions, but if you apply the modifier separately from the structure, the modifier will take the body and our instance will remain empty. When we call it again we will get the `ptr...` error. 

That's why such a check mechanism is implemented for `wasm` and other languages.
First you create an analog of the structure for the language that will use it, in `wasm` it looks like this

```rust
#[wasm_bindgen(js_name = "Rule")]
#[derive(Debug, Default, Serialize, Deserialize)]]
pub struct WasmRule(pub(crate) Option<Rule>);
```

Our `option`, will be a kind of flag that there is a value or not. That is, the value is either `None` or `Rule`.

Now let's implement the functionality that translates from borrowing to owning (when `try_from` is implemented, `try_into` is automatically implemented)

```rust
impl TryFrom<&mut WasmRule> for WasmRule {
    type Error = JsValue;

    fn try_from(value: &mut WasmRule) -> Result<Self, Self::Error> {
        if value.0.is_some() {
            Ok(std::mem::take(value))
        } else {
            Err(JsValue::from_str(ERR_OPTION_RULE))
        }
    }
}
```

Here we see `std::mem::take()`, it will take the value and leave the standard value in its place in our case it is `None`,
and now make an analog of `counter_more_than(x)` functions for `wasm` versions

``` rust
pub fn counter_less_than(&mut self, count: usize) -> Result<WasmRule, JsValue> {
        let mut mem_self: WasmRule = self.try_into()?
        mem_self.0 = mem_self.0.map(|rule| rule.counter_less_than(count));
        Ok(mem_self)
    }
```


This allows you to efficiently manage the state of objects without having to create unnecessary copies of the data.