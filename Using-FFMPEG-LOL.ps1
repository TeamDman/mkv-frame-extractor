$input_video = Get-Content -Raw .\file.txt
New-Item -ItemType Directory -ErrorAction SilentlyContinue -Path out-ffmpeg
ffmpeg -i "$input_video" -vf "fps=1" out-ffmpeg/frame%04d.png