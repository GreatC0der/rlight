use std::process::Command;
use std::thread;
use std::time::Duration;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::Device;

mod config;
use config::load_config;

fn main() {
    let config = load_config();
    let device = Device::new(0).expect("Failed to open device");
    let delay = Duration::from_secs(config.delay);
    loop {
        let mut stream = create_stream(&device);
        // Getting a picture from the camera.
        let (buf, _) = stream.next().unwrap();
        // Calculating avarage brightness.
        let avr_br = (sum(buf) / buf.len()) as f64 / config.darkness_sensetivity;
        // Dropping the stream so the led turns off.
        drop(stream);
        // Changing screen brightness.
        println!("Brightness: {}", avr_br);
        let mut bash = Command::new("/bin/bash");
        let error = bash
            .arg("-c")
            .arg(format!("{} {}", config.set_brightness_cmd, avr_br))
            .spawn()
            .expect("Couldn't execute a command to change the brightness")
            .stderr;

        if error.is_some() {
            panic!("Failed to change the brightness.");
        }

        thread::sleep(delay);
    }
}

fn sum(slice: &[u8]) -> usize {
    let mut result = 0;
    for number in slice {
        result += *number as usize;
    }
    result
}

fn create_stream<'a>(device: &Device) -> Stream<'a> {
    Stream::with_buffers(device, Type::VideoCapture, 1).expect("Failed to create buffer stream")
}
