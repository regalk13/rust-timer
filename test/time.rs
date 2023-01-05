use std::{thread, time};

fn convert_time(mut seconds: u32) -> (u32, u32, u32) {
    let hour = seconds / 3600;
    seconds = seconds % 3600;
    let minutes = seconds / 60;
    seconds = seconds % 60;
    (hour, minutes, seconds)
}
fn main() {
    const DELTA_TIME: f64 = 1.0 / 60.0;

    let mut result = 12345;
    loop {
        let time = convert_time(result);
        println!("{}:{}:{}", time.0, time.1, time.2);
        println!("The delta time is: {}", DELTA_TIME);

        // Add a second
        result += 1;
        let ten_millis = time::Duration::from_secs((1.0 * 60.0 * DELTA_TIME) as u64);
        thread::sleep(ten_millis);
    }
}
