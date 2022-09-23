use std::{thread, time::Duration};

use cpplib;
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
fn main() {
    println!("1+1={}", cpplib::add(1, 1));

    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let handle = chip
        .get_line(65)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "out")
        .unwrap();

    loop {
        thread::sleep(Duration::from_millis(500));
        handle.set_value(1).unwrap();
        thread::sleep(Duration::from_millis(500));
        handle.set_value(0).unwrap();
    }
}
