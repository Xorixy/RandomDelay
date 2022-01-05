use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread::sleep;
use std::env;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let max_delay: u64 = match args.get(1) {
        Some(input) => {
            match input.parse::<u64>() {
                Ok(num) => num,
                Err(_e) => 60,
            }
        }
        None => {
            //println!("No input argument given, delay set to default");
            60
        },
    };

    let now = SystemTime::now();
    let delay = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64 % max_delay + 1;
    
    // we sleep for 2 seconds
    sleep(Duration::new(delay, 0));
    match now.elapsed() {
        Ok(elapsed) => {
        // it prints '2'
            println!("Time elapsed: {} seconds.", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
}
