use brightness::blocking::Brightness as _;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::CameraIndex::Index;
use nokhwa::utils::{RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use std::cmp::min;
use std::thread;
use std::time::Duration;

mod config;
use config::load_config;

use crate::config::Config;

fn main() {
    let mut config = load_config();
    println!(
        "Config path: {}",
        confy::get_configuration_file_path("rlight", None)
            .unwrap()
            .to_string_lossy()
    );

    let learning_coefficient = config.learning_coefficient;
    let delay = Duration::from_secs(config.delay);

    let brightness_d = brightness::blocking::brightness_devices()
        .next()
        .unwrap()
        .unwrap();

    let camera = get_camera(config.camera);
    let buf_len = (camera.resolution().width() * camera.resolution().height()) as usize;
    let buf_indexes: Vec<usize> = (0..buf_len).filter(|x| x % config.step == 0).collect();
    let checked_buf_length = buf_indexes.len();

    loop {
        let mut camera = get_camera(config.camera);

        // Getting a picture from the camera.
        camera.open_stream().unwrap();
        let frame = camera.frame().unwrap();
        let buf = frame.buffer();

        // Calculating average brightness.
        let raw_avrg_br = calc_average(buf, &buf_indexes, checked_buf_length);

        camera.stop_stream().unwrap();
        drop(camera);

        // Changing screen brightness.
        let sensitivity = get_sensitivity(&raw_avrg_br, &mut config);

        let avrg_br = min(100, (raw_avrg_br * *sensitivity) as u8);
        brightness_d.set(avrg_br as u32).unwrap();
        println!("Changed brightness to {}", avrg_br);

        // Waiting
        thread::sleep(delay);

        /*
        Changing sensitivity
        If: adaptive sensitivity enabled and brightness has been changed manualy.
        */
        if config.adaptive_sensitivity {
            let current_br = brightness_d.get().unwrap() as u8;
            if current_br != avrg_br {
                let sensitivity = get_sensitivity(&raw_avrg_br, &mut config);
                *sensitivity -= (avrg_br as i8 - current_br as i8) as f32 * learning_coefficient;
                if sensitivity.is_sign_negative() {
                    *sensitivity = Config::default().mid_sensitivity;
                }
                println!("Sensetivity changed. Now it is {}", *sensitivity);
                config.save();
            }
        }
    }
}

fn calc_average(slice: &[u8], slice_indexes: &Vec<usize>, total: usize) -> f32 {
    let mut result = 0;
    for i in slice_indexes {
        result += slice[*i] as usize;
    }
    result as f32 / total as f32
}

fn get_sensitivity<'a>(raw_avrg_br: &f32, config: &'a mut Config) -> &'a mut f32 {
    match *raw_avrg_br as u8 {
        0..=84 => &mut config.dark_sensitivity,
        85..=169 => &mut config.mid_sensitivity,
        170..=255 => &mut config.light_sensitivity,
    }
}

fn get_camera(camera: u32) -> Camera {
    Camera::new(
        Index(camera),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
    )
    .unwrap()
}
