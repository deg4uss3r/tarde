use std::time;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not convert negative number to time duration: {}", number))]
    Convert{
        number: i128
    },
    
    #[snafu(display("Could not convert 128-bit to 64-bit number (overflow error): {}", number))]
    OverFlow{
        number: u128
    }
}

pub trait Duration {
    fn to_us(self) -> Result<time::Duration, Error>;
    fn to_ms(self) -> Result<time::Duration, Error>;
    fn to_ns(self) -> Result<time::Duration, Error>;
    fn to_sec(self) -> Result<time::Duration, Error>;
}

impl Duration for u8 {
    fn to_us(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self.into()))
    } 

    fn to_ms(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self.into()))
    }

    fn to_ns(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self.into()))
    }

    fn to_sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self.into()))
    }
}

impl Duration for u16 {
    fn to_us(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self.into()))
    } 

    fn to_ms(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self.into()))
    }

    fn to_ns(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self.into()))
    }

    fn to_sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self.into()))
    }
}

impl Duration for u32 {
    fn to_us(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self.into()))
    } 

    fn to_ms(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self.into()))
    }

    fn to_ns(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self.into()))
    }

    fn to_sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self.into()))
    }
}

impl Duration for u64 {
    fn to_us(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_micros(self))
    } 

    fn to_ms(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_millis(self))
    }

    fn to_ns(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_nanos(self))
    }

    fn to_sec(self) -> Result<time::Duration, Error> {
        Ok(time::Duration::from_secs(self))
    }
}

impl Duration for u128 {
    fn to_us(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_micros(self as u64))
        }
    } 
    fn to_ms(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_millis(self as u64))
        }
    } 
    fn to_ns(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_nanos(self as u64))
        }
    } 
    fn to_sec(self) -> Result<time::Duration, Error> {
        if self > u64::max_value() as u128 {
            Err(Error::OverFlow{number: self})
        } else {
            Ok(time::Duration::from_secs(self as u64))
        }
    } 
}

impl Duration for i8 {
    fn to_us(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ms(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ns(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}


impl Duration for i16 {
    fn to_us(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ms(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ns(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}


impl Duration for i32 {
    fn to_us(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ms(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ns(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}

impl Duration for i64 {
    fn to_us(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_micros(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ms(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_millis(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_ns(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_nanos(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
    fn to_sec(self) -> Result<time::Duration, Error> {
        if self > 0 {
            Ok(time::Duration::from_secs(self as u64))
        } else {
             Err(Error::Convert{number: self as i128})
        }
    }
}

impl Duration for i128 {
    fn to_us(self) -> Result<time::Duration, Error> {
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
    fn to_ms(self) -> Result<time::Duration, Error> {
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
    fn to_ns(self) -> Result<time::Duration, Error> {
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
    fn to_sec(self) -> Result<time::Duration, Error> {
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
