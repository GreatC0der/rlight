use std::process::Command;

use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::Device;

pub fn create_stream<'a>(device: &Device) -> Stream<'a> {
    Stream::with_buffers(device, Type::VideoCapture, 1).expect("Failed to create buffer stream")
}

pub fn change_brightness(set_brightness_cmd: &str, brightness: f64) {
    let mut bash = Command::new("/bin/bash");
    let error = bash
        .arg("-c")
        .arg(format!("{} {}", set_brightness_cmd, brightness))
        .spawn()
        .expect("Couldn't execute a command to change the brightness")
        .stderr;

    if error.is_some() {
        panic!("Failed to change the brightness.");
    }
}
