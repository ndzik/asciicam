mod canvas;

use canvas::canvas::{Canvas, Dim};
use canvas::terminal::terminal::*;
use clap::{arg, Command};
use log::info;
use nokhwa::{Camera, CaptureAPIBackend, FrameFormat};
use simple_logger::SimpleLogger;
use std::time::Instant;

fn translate_rgb(
    chars: &[u8],
    canvas: &dyn Canvas,
    buffer: &[u8],
    cam_width: usize,
    cam_height: usize,
) -> Vec<char> {
    let Dim {
        width: canvas_width,
        height: canvas_height,
    } = canvas.dim();
    let mut asciibuffer = vec![' '; (canvas_width * canvas_height) as usize];

    let pixel_per_cell_y = cam_height / canvas_height as usize;
    let pixel_per_cell_x = cam_width / canvas_width as usize;

    let char_for_greyscale = |cur_x: usize, cur_y: usize| {
        let local_x = cur_x * pixel_per_cell_x * 3;
        let local_y = cur_y * pixel_per_cell_y * 3;
        let mut pixels = Vec::with_capacity(pixel_per_cell_x * pixel_per_cell_y);
        for y in (local_y..(local_y + pixel_per_cell_y * 3)).step_by(3) {
            for x in (local_x..(local_x + pixel_per_cell_x * 3)).step_by(3) {
                let r = buffer[cam_width * y + x];
                let g = buffer[cam_width * y + x + 1];
                let b = buffer[cam_width * y + x + 2];
                let greyscale = (r as u32 + g as u32 + b as u32) / 3;
                pixels.push(greyscale as u32);
            }
        }
        let sum: u32 = pixels.iter().sum();
        let average: u8 = (sum / (pixels.len() as u32)) as u8;
        let index: usize =
            (average as f64 * (((chars.len() - 1) as f64) / ((0xff) as f64))) as usize;
        chars[index] as char
    };

    for row in 0..canvas_height {
        for col in 0..canvas_width {
            asciibuffer[(row * canvas_width + col) as usize] =
                char_for_greyscale(col as usize, row as usize);
        }
    }

    asciibuffer
}

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const FPS: u32 = 30;

fn main() -> Result<(), std::io::Error> {
    let matches = Command::new("asciicam")
        .version("0.0.1")
        .author("ndzik <norbert@perun.network>")
        .about("Commandline application to view your ASCII encoded webcam feed")
        .arg(arg!(-l - -log).required(false))
        .get_matches();

    if matches.is_present("log") {
        SimpleLogger::new().init().unwrap();
    }

    let chars: Vec<u8> =
        b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:.                                      "
            .iter()
            .rev()
            .cloned()
            .collect();

    let mut camera = Camera::new_with(
        0,
        WIDTH,
        HEIGHT,
        FPS,
        FrameFormat::MJPEG,
        CaptureAPIBackend::Video4Linux,
    )
    .unwrap();

    camera.open_stream().unwrap();
    info!("camera resolution: {}", camera.resolution());

    let mut canvas = TerminalCanvas::new()?;

    loop {
        let before_capture = Instant::now();
        let frame = match camera.frame() {
            Ok(frame) => frame,
            Err(why) => panic!("error fetching frame from source: {}", why),
        };
        let after_capture = Instant::now();

        let (dimx, dimy) = frame.dimensions();
        info!("frame dimension: {}x{}", dimx, dimy);
        info!("frame length: {}", frame.len());

        let before_translation = Instant::now();
        let ascii_image = translate_rgb(
            &chars,
            &canvas,
            &frame.as_raw(),
            WIDTH as usize,
            HEIGHT as usize,
        );
        let after_translation = Instant::now();

        let before_draw = Instant::now();
        if let Err(err) = canvas.draw(&ascii_image) {
            panic!("error drawing buffer: {}", err);
        }
        let after_draw = Instant::now();

        info!(
            "capture time: {}",
            after_capture.duration_since(before_capture).as_millis()
        );
        info!(
            "translation time: {}",
            after_translation
                .duration_since(before_translation)
                .as_millis()
        );
        info!(
            "draw time: {}",
            after_draw.duration_since(before_draw).as_millis()
        );
        info!(
            "frame time: {}",
            after_draw.duration_since(before_capture).as_millis()
        );
    }
}
