use std::time::Duration;
use std::{cmp::min, thread};
use v4l::{io::traits::CaptureStream, Device};

mod config;
use config::load_config;
mod io;
use io::{change_brightness, create_stream, get_brightness};

use crate::config::Config;
use at_debug::at_debug;

fn main() {
    at_debug!(println!(
        "You are debugging it! So trying to improve it. Thank you."
    ));

    let mut config = load_config();
    let device = Device::new(0).expect("Failed to open device.");
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
        let (buf, _) = stream.next().unwrap();
        // Calculating avarage brightness.
        let avrg_br = min(
            100,
            (calc_avarage(buf, &buf_indexes, checked_buf_length) * config.sensetivity) as u8,
        );
        // Dropping the stream so the led turns off.
        drop(stream);
        // Changing screen brightness.
        at_debug!(println!("Brightness has been changed to {}%.", avrg_br));
        change_brightness(&config.set_brightness_cmd, avrg_br);

        // Waiting
        thread::sleep(delay);

        /*
        Changing sensetivity
        If: adaptive sensetivity enabled and brightness has been changed manualy.
        */
        if config.adaptive_sensetivity {
            let current_br = get_brightness(&config.get_brightness_cmd);
            if current_br != avrg_br {
                println!("Changing sensetivity because the brightness was changed manually.");
                println!(
                    "Suggested brightness was {}%, Current brightness is {}%.",
                    avrg_br, current_br
                );
                println!("Old sensetivity was {}%.", config.sensetivity);

                config.sensetivity -= (avrg_br as i8 - current_br as i8) as f32 * 0.001;

                if config.sensetivity.is_sign_negative() {
                    config.sensetivity = Config::default().sensetivity;
                }

                println!("New sensetivity is {}%.", config.sensetivity);
                config.save();
            }
        }
        println!("-------------");
    }
}

fn calc_avarage(slice: &[u8], slice_indexes: &Vec<usize>, total: usize) -> f32 {
    let mut result = 0;
    for i in slice_indexes {
        result += slice[*i] as usize;
    }
    result as f32 / total as f32
}
