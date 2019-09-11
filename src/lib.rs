//! TARDE (tar-dee, Time And Rust Duration Ergonomics 
//! A simple wrapper to '{integer}' types that improves ergonomics around `std::time::Duration` 
//!
//! Example:
//! ```rust
//! use std::time::{Instant, Duration};
//! use std::thread;
//! use tarde::*;
//!
//! // Before:
//! fn std_way() {
//!     let x: u64 = 10;
//!     let now = Instant::now();
//!     thread::sleep(Duration::from_millis(2569));
//!
//!     assert!(now.elapsed() >= Duration::from_millis(x));
//! }
//! 
//! // After:
//! fn main() -> Result<(), Error> {
//!    let x = 10;
//!    let now = Instant::now();
//!    thread::sleep(x.millis()?); 
//!
//!    Ok(assert!(now.elapsed() >= x.millis()?))
//! }
//! ```
//!

use std::time;
use snafu::Snafu;

/// Using [`snafu`](https://docs.rs/snafu/0.5.0/snafu/) for custom error `enum` 
/// This `enum` `Error` contains two types; `Convert`, and `OverFlow`
#[derive(Debug, Snafu)]
pub enum Error {
    /// `Error::Convert` type will output `("Could not convert negative number to time duration: {}", number)`
    /// This error happens when a negative signed number (e.g. `-8_i16`) is passed as input to a
    /// to a `During` trait implementation.
    #[snafu(display("Could not convert negative number to time duration: {}", number))]
    Convert{
        number: i128
    },
    
    /// `Error::OverFlow` type will output `("Could not convert 128-bit to 64-bit number (overflow error): {}", number))` 
    /// This error happens when too large of a `u128` number was passed as input to
    /// `std::time::Duration` as it only accepts `u64` as input to the `During` initializers 
    #[snafu(display("Could not convert 128-bit to 64-bit number (overflow error): {}", number))]
    OverFlow{
        number: u128
    }
}

/// `During` is the trait we are implementing over Rust [primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
/// `During` implements converting primitives to `micros`, `millis`, `nanos`, and `secs` for
/// `Duration` initialization.
pub trait During {
    fn micros(self) -> Result<time::Duration, Error>;
    fn millis(self) -> Result<time::Duration, Error>;
    fn nanos(self) -> Result<time::Duration, Error>;
    fn sec(self) -> Result<time::Duration, Error>;
}

impl During for u8 {
    fn micros(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self.into()))
    } 

    fn millis(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self.into()))
    }

    fn nanos(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self.into()))
    }

    fn sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self.into()))
    }
}

impl During for u16 {
    fn micros(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self.into()))
    } 

    fn millis(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self.into()))
    }

    fn nanos(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self.into()))
    }

    fn sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self.into()))
    }
}

impl During for u32 {
    fn micros(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self.into()))
    } 

    fn millis(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self.into()))
    }

    fn nanos(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self.into()))
    }

    fn sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self.into()))
    }
}

impl During for u64 {
    fn micros(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self))
    } 

    fn millis(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self))
    }

    fn nanos(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self))
    }

    fn sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self))
    }
}

impl During for u128 {
    fn micros(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_micros(self as u64))
        }
    } 
    fn millis(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_millis(self as u64))
        }
    } 
    fn nanos(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_nanos(self as u64))
        }
    } 
    fn sec(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_secs(self as u64))
        }
    } 
}

impl During for i8 {
    fn micros(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn millis(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn nanos(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}


impl During for i16 {
    fn micros(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn millis(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn nanos(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}


impl During for i32 {
    fn micros(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn millis(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn nanos(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}

impl During for i64 {
    fn micros(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn millis(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn nanos(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}

impl During for i128 {
    fn micros(self) -> Result<time::Duration, Error> {
        if self > 0 {
            if self > u64::max_value() as i128 {
                Err(Error::OverFlow{number: self as u128})
            } else {
                Ok(time::Duration::from_micros(self as u64))
            }
        } else {
            Err(Error::Convert{number:self})
        }
    }
    fn millis(self) -> Result<time::Duration, Error> {
        if self > 0 {
            if self > u64::max_value() as i128 {
                Err(Error::OverFlow{number: self as u128})
            } else {
                Ok(time::Duration::from_millis(self as u64))
            }
        } else {
            Err(Error::Convert{number:self})
        }
    }
    fn nanos(self) -> Result<time::Duration, Error> {
        if self > 0 {
            if self > u64::max_value() as i128 {
                Err(Error::OverFlow{number: self as u128})
            } else {
                Ok(time::Duration::from_nanos(self as u64))
            }
        } else {
            Err(Error::Convert{number:self})
        }
    }
    fn sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            if self > u64::max_value() as i128 {
                Err(Error::OverFlow{number: self as u128})
            } else {
                Ok(time::Duration::from_secs(self as u64))
            }
        } else {
            Err(Error::Convert{number:self})
        }
    }
}
