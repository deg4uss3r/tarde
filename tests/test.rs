use tarde::Duration;
use std::{thread, time};

#[test]
fn test_milliseconds() -> Result<(), tarde::Error>{
    let x = 10;
    let now = time::Instant::now();
    thread::sleep(x.to_ms()?); 

    Ok(assert!(now.elapsed() >= x.to_ms()?))
}

#[test]
#[should_panic]
fn test_milliseconds_overflow() {
    let x: u128 = 184467440737095516199999;
    x.to_ms().unwrap(); 
} 

#[test]
#[should_panic]
fn test_negative_milliseconds() {
    let x: i32 = -42;
    x.to_ms().unwrap(); 
}

#[test]
fn test_microseconds() -> Result<(), tarde::Error> {
    let x = 10; 
    let now = time::Instant::now();
    thread::sleep(x.to_us()?);
    
    Ok(assert!(now.elapsed() >= x.to_us()?))
}

#[test]
#[should_panic]
fn test_microseconds_overflow() {
    let x: u128 = 18446744073709551614599999;
    x.to_us().unwrap();
}

#[test]
#[should_panic]
fn test_microseconds_negative() {
    let x: i32 = -42;
    x.to_us().unwrap();
}

#[test]
fn test_nanoseconds() -> Result<(), tarde::Error> {
    let x = 10; 
    let now = time::Instant::now();
    thread::sleep(x.to_ns()?);
    
    Ok(assert!(now.elapsed() >= x.to_ns()?))
}

#[test]
#[should_panic]
fn test_nanoseconds_overflow() {
    let x: u128 = 18446744073709551614599999;
    x.to_ns().unwrap();
}

#[test]
#[should_panic]
fn test_nanoseconds_negative() {
    let x: i32 = -42;
    x.to_ns().unwrap();
}

#[test]
fn test_seconds() -> Result<(), tarde::Error> {
    let x = 1; 
    let now = time::Instant::now();
    thread::sleep(x.to_sec()?);
    
    Ok(assert!(now.elapsed() >= x.to_sec()?))
}

#[test]
#[should_panic]
fn test_seconds_overflow() {
    let x: u128 = 18446744073709551614599999;
    x.to_sec().unwrap();
}

#[test]
#[should_panic]
fn test_seconds_negative() {
    let x: i32 = -42;
    x.to_sec().unwrap();
}
