use std::time::Instant;
use std::thread;
use tarde::{During, Error};

fn main() -> Result<(), Error> {
    let x = 42.millis()?;
    let now = Instant::now();
    thread::sleep(x); 

    println!("now.elapsed: {:#?}", now.elapsed());

    Ok(assert!(now.elapsed() >= x))
}
