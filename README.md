# inputlib

A small [crate](https://crates.io/crates/inputlib) that provides Python-style input handling with a convenient `input!` macro.

## Example

```rust
use inputlib::input;

fn main() {
    let name = input!("What's your name? ");
    println!("Hello, {}!", name);
}
```
