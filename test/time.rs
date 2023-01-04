fn main() {
    const DELTA_TIME: f64 = 1.0 / 60.0;
    let seconds = 2.0;
    let minutes = 5.0;

    let mut result = 0.0;
    result += seconds;
    result += minutes * 60.0;
    loop {
        let minutes = result / 60.0 % 60.0;
        println!("{}:{}", minutes / 10.0, minutes % 10.0);
        if result > 1e-6 {
            result -= DELTA_TIME;
        } else {
            result = 0.0;
            println!("We end!");
            break;
        }
    }
}
