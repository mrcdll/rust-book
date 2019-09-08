use std::io;

fn main() {
    convert_temperature()
}

fn convert_temperature() {
    println!("Enter temperature");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let len = temperature.len();
    temperature.truncate(len - 1);

    let unit = temperature.remove(temperature.len() - 1);

    println!("this is the unit {} and value {}", &unit, &temperature);
}
