## fftool

I made this in one quick evening as a little helper tool for myself
for a personal project that involves remuxing and transcoding tons
of disparate types of video files.

It's more or less an exercise to warm up with Rust and is definitely
a pretty cruddy little program with iffy best-rust-practices.

### Usage

`ffmpeg` and `ffprobe` must be in your shell's PATH.

To use the tool, open a shell in the directory you want to pick a video file from and run `fftool`.
It will only show you input files that have both at least 1 audio & video stream, and the interactive
questions will guide you through the rest.

```
fftool.exe
Found ffmpeg friendly files:
  0: [Judas] Initial D - S01E17.mkv
  1: [Judas] Initial D - S01E14.mkv
Pick which file to remux: 1
You picked '1'
Operating on .\[Judas] Initial D - S01E14.mkv...
  0: opus - "[Judas] JAP Stereo (Opus 112Kbps)"
  1: opus - "[Judas] ENG Stereo (Opus 112Kbps)"
Pick audio stream:0
Output file name: initial_d_ep14_english.mkv
RUN PARAMETERS:
 video: `0:hevc`
 audio: `1:opus`
 in-file: `.\[Judas] Initial D - S01E14.mkv` out-file: `initial_d_ep14_english.mkv`
Good to go? ('y' to continue)
```

When you type `y` and hit enter, `fftool` will show the full output of the ffmpeg command and then exit.

NOTE: If the file only has 1 stream of a given type, it will be auto-picked.

### Example Scenario

Say you have a `.mkv` file with a bunch of streams:
```
doki-doki-anime-is-bad.mkv
- 0: video, h264
- 1: ass, "english subtitlse"
- 2: ass, "german subtitlse"
- 3: audio, japanese
- 4: audio, english
- 5: audio, german
```

And you want to instead have a final video file that is only 1 video and 1 audio stream:

```
doki-doki-anime-is-bad.mkv
- 0: video, h264
- 1: audio, japanese
```

We can losslessly [remux](https://scenelingo.wordpress.com/2015/09/09/what-does-remux-mean/#:~:text=A%20remux%20is%20a%20rip,quality%20as%20on%20original%20disc.)
these streams with the suite of tools available to us in [ffmpeg](http://ffmpeg.org/). Namely: `ffprobe` to show streams, and `ffmpeg` to do the muxing.

For humans, this is a little tedious if you need to do this a bunch of times,
you first have to use `ffprobe` to see and understand which streams to pick, then use ffmpeg
like so to remux:

`ffmpeg -i doki.mkv -c:v copy -c:a copy -map 0:$X -map 0:$Y output.mkv`

(where `X`, `Y` are the video stream index and audio stream index, respectively).

While there's a million ways to solve a simple problem like this, I chose to write a little interactive
rust program as an exercise.