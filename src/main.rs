use ffmpeg_next as ffmpeg;
use image::{ImageBuffer, RgbImage};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize FFmpeg
    ffmpeg::init()?;

    // Read the input file path from "file.txt"
    let file_path = fs::read_to_string("file.txt")?.trim().to_string();

    // Open the input video file
    let mut ictx = ffmpeg::format::input(&file_path)?;

    // Find the best video stream
    let input = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or("Could not find video stream")?;

    let video_stream_index = input.index();
    let codec_parameters = input.parameters();
    let mut decoder = ffmpeg::codec::context::Context::from_parameters(codec_parameters)?
        .decoder()
        .video()?;

    let time_base = input.time_base();
    let mut frame_index = 0;

    // Prepare output directory
    let output_dir = Path::new("out-mine");
    fs::create_dir_all(output_dir)?;

    // Create a scaler to convert frames to RGB24
    let mut scaler = ffmpeg::software::scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg::software::scaling::Flags::BILINEAR,
    )?;

    // Allocate frames
    let mut decoded = ffmpeg::util::frame::Video::empty();
    let mut rgb_frame = ffmpeg::util::frame::Video::empty();

    // Read packets and decode frames
    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;

            while decoder.receive_frame(&mut decoded).is_ok() {
                // Scale the frame to RGB24
                scaler.run(&decoded, &mut rgb_frame)?;

                // Calculate frame timestamp in seconds
                let pts = decoded.pts().unwrap_or(0);
                let time_in_seconds =
                    pts as f64 * time_base.0 as f64 / time_base.1 as f64;

                // Save frames at 1-second intervals (within a tolerance)
                if time_in_seconds.fract() < 0.01 {
                    let filename = output_dir.join(format!("frame_{:04}.png", frame_index));
                    save_frame_as_image(&rgb_frame, &filename)?;
                    println!("Saved {}", filename.display());
                    frame_index += 1;
                }
            }
        }
    }

    // Flush the decoder
    decoder.send_eof()?;
    while decoder.receive_frame(&mut decoded).is_ok() {
        // Scale the frame to RGB24
        scaler.run(&decoded, &mut rgb_frame)?;

        // Calculate frame timestamp in seconds
        let pts = decoded.pts().unwrap_or(0);
        let time_in_seconds =
            pts as f64 * time_base.0 as f64 / time_base.1 as f64;

        // Save frames at 1-second intervals (within a tolerance)
        if time_in_seconds.fract() < 0.01 {
            let filename = output_dir.join(format!("frame_{:04}.png", frame_index));
            save_frame_as_image(&rgb_frame, &filename)?;
            println!("Saved {}", filename.display());
            frame_index += 1;
        }
    }

    Ok(())
}

fn save_frame_as_image(
    frame: &ffmpeg::util::frame::Video,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the frame is in RGB24 format
    if frame.format() != ffmpeg::format::Pixel::RGB24 {
        return Err("Frame is not in RGB24 format".into());
    }

    let width = frame.width() as u32;
    let height = frame.height() as u32;

    // Access the data
    let data = frame.data(0);

    // Create an image buffer from raw data
    let img_buffer: RgbImage = ImageBuffer::from_raw(width, height, data.to_vec())
        .ok_or("Failed to create image buffer")?;

    // Save the image as PNG
    img_buffer.save(path)?;

    Ok(())
}
