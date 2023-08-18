use std::process::Command;

use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::Device;

pub fn create_stream<'a>(device: &Device) -> Stream<'a> {
    Stream::with_buffers(device, Type::VideoCapture, 1).expect("Failed to create buffer stream")
}

pub fn change_brightness(cmd: &str, brightness: u8) {
    let mut bash = Command::new("/bin/bash");
    let error = bash
        .arg("-c")
        .arg(format!("{} {}", cmd, brightness))
        .spawn()
        .expect("Couldn't change the brightness.")
        .stderr;

    if error.is_some() {
        panic!("Failed to change the brightness.");
    }
}

pub fn get_brightness(cmd: &str) -> u8 {
    let mut bash = Command::new("/bin/bash");
    let output = bash
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to get the brightness.");
    let str_number = String::from_utf8_lossy(&output.stdout);

    str_number
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<f32>()
        .expect("Failed to convert the output of `get_brightness_cmd` to a number.") as u8
}
