# mkv-frame-extractor

DISCLAIMER - THIS CODE IS WRITTEN BY AI PRETTY MUCH LOL

Using `ffmpeg -i "$input_video" -vf "fps=1" out-ffmpeg/frame%04d.png` took 45 seconds to write 1435 frames.

Using `cargo run` took 3m31s to write 471 frames but all the frames were garbage lol.

Conclusion: the ai written code using `ffmpeg-next` is doing something wrong lol.

## Installing

### Rust-ffmpeg

#### FFMPEG

1. Read https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building

2. Download FFMPEG from https://github.com/BtbN/FFmpeg-Builds/releases

3. Set FFMPEG_DIR to the extracted dir

4. Add that dir to path