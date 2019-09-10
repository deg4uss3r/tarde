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
fn test_large_milliseconds() -> Result<(), tarde::Error>{
    let x: u128 = 18446744073709551615;
    let duration_x = x.to_ms()?; 
    Ok(assert_eq!(duration_x, time::Duration::from_millis(u64::max_value())))
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
    
    assert!(now.elapsed() >= x.to_us()?);

    Ok(())
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
