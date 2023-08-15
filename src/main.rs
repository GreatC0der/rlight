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

    // Taking a picture so we know the size
    let mut stream = create_stream(&device);
    let (buf, _) = stream.next().unwrap();
    let buf_len = buf.len();
    drop(stream);

    // Calculating buf_indexes
    let buf_indexes: Vec<usize> = (0..buf_len).filter(|x| x % config.step == 0).collect();

    let checked_buf_length = buf_indexes.len();

    loop {
        let mut stream = create_stream(&device);
        // Getting a picture from the camera.
        // Calculating avarage brightness.
        let (buf, _) = stream.next().unwrap();
        let avr_br =
            calc_avarage(buf, &buf_indexes, checked_buf_length) / config.darkness_sensetivity;
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

fn calc_avarage(slice: &[u8], slice_indexes: &Vec<usize>, total: usize) -> f64 {
    let mut result = 0;
    for i in slice_indexes {
        result += slice[*i] as usize;
    }
    result as f64 / total as f64
}

fn create_stream<'a>(device: &Device) -> Stream<'a> {
    Stream::with_buffers(device, Type::VideoCapture, 1).expect("Failed to create buffer stream")
}
