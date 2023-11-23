use std::time::{Duration, Instant};

fn main() {
    const TOLLERANCE: f64 = 1e-323;
    println!("Tollerance set to {}", TOLLERANCE.);
    // following some stack exchange answer on the webz I need to setup sum variables
    const NUMBER: i128 = 22222222222222222222222222; 
    let mut guess: f64 = 1.0;
    let mut count: i128 = 0;
    let start = Instant::now();
    loop {
        count += 1;
        let root = 0.5 * (guess + NUMBER as f64 / guess);
        if (root - guess).abs() < TOLLERANCE {
            println!("Square root: {}", root);
            println!("Root^2 = {}", root * root);
            break;
        }
        guess = root;
    }

    let duration = start.elapsed();
    println!("In seconds: {}", duration.as_secs());
    println!("In milliseconds: {}", duration.as_millis());
    println!("In microseconds: {}", duration.as_micros());
    println!("In nanoseconds: {}", duration.as_nanos());
}
