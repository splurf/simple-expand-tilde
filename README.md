# simple-expand-tilde
[![Crate](https://img.shields.io/crates/v/simple-expand-tilde.svg)](https://crates.io/crates/simple-expand-tilde)

An extremely tiny and reliable Rust library that parses [tilde expansion](https://www.gnu.org/software/bash/manual/html_node/Tilde-Expansion.html).

## Usage
```rust
use simple_expand_tilde::*;

fn main() {
    // Windows => "C:\Users\<USER>\.rustup"
    // Linux   => "/home/<USER>/.rustuup"
    // Mac     => "/Users/<USER>/.rustup"
    let path = expand_tilde("~/.rustup").unwrap();
    println!("{:?}", path)
}
```