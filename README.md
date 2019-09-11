# TARDE

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/deg4uss3r/tarde/blob/master/LICENSE.md) 
[![Crates.io](https://img.shields.io/crates/v/tarde.svg)](https://crates.io/crates/tarde)

Linux: [![Build Status](https://travis-ci.org/deg4uss3r/tarde.svg?branch=master)](https://travis-ci.org/deg4uss3r/tarde)

Windows: [![Build Status](https://ci.appveyor.com/api/projects/status/ejg8c33dn31nhv36/branch/master?svg=true)](https://ci.appveyor.com/project/deg4uss3r/tarde/branch/master)

TARDE (tar-dee, Time And Rust Duration Ergonomics) is a small library to allow for better ergonomics when using `std::time::Duration`.

## Example

```rust
use std::time::Instant;
use tarde::{During, Error};

fn main() -> Result<(), tarde::Error> {
    let now = time::Instant::now();
    thread::sleep(10.millis()?); 

    Ok(assert!(now.elapsed() >= 10.millis()?))
}
```

