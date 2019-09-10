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
    fn micros(self) -> Result<time::Duration, Error>;
    fn millis(self) -> Result<time::Duration, Error>;
    fn nanos(self) -> Result<time::Duration, Error>;
    fn sec(self) -> Result<time::Duration, Error>;
}

impl Duration for u8 {
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

impl Duration for u16 {
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

impl Duration for u32 {
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

impl Duration for u64 {
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

impl Duration for u128 {
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

impl Duration for i8 {
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


impl Duration for i16 {
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


impl Duration for i32 {
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

impl Duration for i64 {
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

impl Duration for i128 {
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
