use clearscreen;
use opencv::{core, imgproc, prelude::*, videoio};

fn pixel_to_ascii(pixel: u8) -> String {
    let ascii_chars = " .,:;i1tfLCG08@";
    let ascii_chars_len = ascii_chars.len();
    let pixel_index = pixel as usize * ascii_chars_len / 256;
    ascii_chars.chars().nth(pixel_index).unwrap().to_string()
}

fn main() {
    // Get video path as argument
    let args: Vec<String> = std::env::args().collect();
    let video_path = &args[1];

    // Load video
    let mut cap = videoio::VideoCapture::from_file(video_path, 0).unwrap();

    let fps = cap.get(videoio::CAP_PROP_FPS).unwrap();

    let frame_durantion = 1000 / fps as i32;

    // let frame_width = cap.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap();
    // let frame_height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap();

    // Get video resolution
    let width: i32 = 150;
    // let height: i32 = ((width * frame_height) / frame_width);
    let height: i32 = 50;

    // maintain aspect ratio
    let mut frame = core::Mat::default();
    let mut gray = core::Mat::default();
    let mut resized_frame = core::Mat::default();

    loop {
        cap.read(&mut frame).unwrap();

        if frame.empty() {
            break;
        }

        // Convert to grayscale
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
        imgproc::resize(
            &gray,
            &mut resized_frame,
            core::Size::new(width, height),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )
        .unwrap();

        let mut ascii_frame: String = String::new();

        for y in 0..height {
            for x in 0..width {
                ascii_frame += &pixel_to_ascii(*resized_frame.at_2d::<u8>(y, x).unwrap());
            }
            ascii_frame += "\n";
        }

        // Clear screen
        clearscreen::clear().unwrap();

        // Print ascii frame
        println!("{}", ascii_frame);

        std::thread::sleep(std::time::Duration::from_millis(
            frame_durantion.try_into().unwrap(),
        ));
    }

    println!(
        "Video resolution: {}x{}@{}fps",
        cap.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap(),
        cap.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap(),
        cap.get(videoio::CAP_PROP_FPS).unwrap()
    );
}
