# TARDE

[![Crates.io](https://img.shields.io/crates/v/tarde.svg)](https://crates.io/crates/tarde)
[![Linux Build](https://travis-ci.org/deg4uss3r/tarde.svg?branch=master)
[![Windows Build](https://ci.appveyor.com/api/projects/status/ejg8c33dn31nhv36/branch/master?svg=true)](https://ci.appveyor.com/project/deg4uss3r/tarde/branch/master)

TARDE (tar-dee, Time And Rust Duration Ergonomics) is a small library to allow for better ergonmics when using `std::time::Duration`.

## Example

```rust
use std::time::Duration;
use tarde::*;

fn main() -> Result<(), tarde::Error> {
    let x = 10;
    let now = time::Instant::now();
    thread::sleep(x.to_ms()?); 

    Ok(assert!(now.elapsed() >= x.to_ms()?))
}
```

