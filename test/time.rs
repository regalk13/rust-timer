use std::{thread, time};

fn convert_time(mut seconds: u32) -> (u32, u32, u32) {
    let hour = seconds / 3600;
    seconds = seconds % 3600;
    let minutes = seconds / 60;
    seconds = seconds % 60;
    (hour, minutes, seconds)
}

fn main() {
    const DELTA_TIME: f64 = 1.0 / 600.0;
    //let mut previous_time = time::Instant::now();
    let mut result = 1;
    loop {
        //let dt = time::Instant::now() - previous_time;
        // previous_time = time::Instant::now();
        let time = convert_time(result);
        println!("{}:{}:{}", time.0, time.1, time.2);
        // println!("The delta time is: {:?}", dt);

        if result >= 10 {
            break;
        }
        // Add a second
        result += (1.0 * 600.0 * DELTA_TIME) as u32;
        // result += DELTA_TIME as u32;
        let ten_seconds = time::Duration::from_secs((1.0 * 600.0 * DELTA_TIME) as u64);
        thread::sleep(ten_seconds);
    }
}
