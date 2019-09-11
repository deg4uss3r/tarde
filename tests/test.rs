use tarde::During;
use std::{thread, time};

#[test]
fn test_milliseconds() -> Result<(), tarde::Error>{
    let x = 10;
    let now = time::Instant::now();
    thread::sleep(x.millis()?); 

    Ok(assert!(now.elapsed() >= x.millis()?))
}

#[test]
#[should_panic]
fn test_milliseconds_overflow() {
    let x: u128 = 184467440737095516199999;
    x.millis().unwrap(); 
} 

#[test]
#[should_panic]
fn test_negative_milliseconds() {
    let x: i32 = -42;
    x.millis().unwrap(); 
}

#[test]
fn test_microseconds() -> Result<(), tarde::Error> {
    let x = 10; 
    let now = time::Instant::now();
    thread::sleep(x.micros()?);
    
    Ok(assert!(now.elapsed() >= x.micros()?))
}

#[test]
#[should_panic]
fn test_microseconds_overflow() {
    let x: u128 = 18446744073709551614599999;
    x.micros().unwrap();
}

#[test]
#[should_panic]
fn test_microseconds_negative() {
    let x: i32 = -42;
    x.micros().unwrap();
}

#[test]
fn test_nanoseconds() -> Result<(), tarde::Error> {
    let x = 10; 
    let now = time::Instant::now();
    thread::sleep(x.nanos()?);
    
    Ok(assert!(now.elapsed() >= x.nanos()?))
}

#[test]
#[should_panic]
fn test_nanoseconds_overflow() {
    let x: u128 = 18446744073709551614599999;
    x.nanos().unwrap();
}

#[test]
#[should_panic]
fn test_nanoseconds_negative() {
    let x: i32 = -42;
    x.nanos().unwrap();
}

#[test]
fn test_seconds() -> Result<(), tarde::Error> {
    let x = 1; 
    let now = time::Instant::now();
    thread::sleep(x.sec()?);
    
    Ok(assert!(now.elapsed() >= x.sec()?))
}

#[test]
#[should_panic]
fn test_seconds_overflow() {
    let x: u128 = 18446744073709551614599999;
    x.sec().unwrap();
}

#[test]
#[should_panic]
fn test_seconds_negative() {
    let x: i32 = -42;
    x.sec().unwrap();
}

#[test]
fn ambiguous_type() -> Result<(), tarde::Error> {
    120.sec()?;

    Ok(())
}
