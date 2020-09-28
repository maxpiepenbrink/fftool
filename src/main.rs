use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, exit};
use std::str;

use ff::*;

mod input_helpers;
mod ff;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_file = match args.get(1) {
        Some(t) => ffprobe_file(Path::new(&t).to_path_buf()).unwrap(),
        None => {
            let video_files = get_all_files();
            do_file_picker(video_files).unwrap()
        }
    };

    println!("Operating on {}...", target_file.path);
    let v_stream = pick_stream(&target_file, "video");
    let a_stream = pick_stream(&target_file, "audio");
    let output_file = input_helpers::get_string_input("Output file name: ");
    println!("RUN PARAMETERS:\n video: `{}`\n audio: `{}`\n in-file: `{}` out-file: `{}`",
             format!("{}:{}", v_stream.index, v_stream.codec_name),
             format!("{}:{}", a_stream.index, a_stream.codec_name),
             target_file.path,
             &output_file
    );

    let choice = input_helpers::get_string_input("Good to go? ('y' to continue)").to_lowercase();
    if choice != "y" { exit(-1) }

    println!("Remuxing...");
    remux(
        &target_file,
        v_stream,
        a_stream,
        output_file,
    );
}

fn remux(file: &VideoFile, v_stream: &CodecStream, a_stream: &CodecStream, out_file: String) {
    let child = Command::new("ffmpeg")
        .args(&[
            "-i", &*file.path,
            "-c:a", "copy",
            "-c:v", "copy",
            "-map", format!("0:{}", v_stream.index).as_str(),
            "-map", format!("0:{}", a_stream.index).as_str(),
            out_file.as_str()
        ])
        .spawn()
        .expect("failed to execute process");

    child.wait_with_output()
        .expect("failed while waiting on remux...");

    println!("Remux complete");
}

fn pick_stream<'a>(file: &'a VideoFile, codec_type: &'a str) -> &'a CodecStream {
    let v_streams = file.streams.iter()
        .filter(|v| v.codec_type == codec_type)
        .collect::<Vec<_>>();

    let stream = match &v_streams.len() {
        1 => v_streams[0],
        _ => {
            for (index, stream) in v_streams.iter().enumerate() {
                let summary = format!("{} - {}", stream.codec_name, stream.tag_title.as_ref().unwrap_or(&"<NO TITLE>".to_string()));
                println!("{:>3}: {}", index, summary)
            }
            v_streams[input_helpers::get_number_input(&*format!("Pick {} stream:", codec_type)) as usize]
        }
    };

    return stream;
}

// i'm sorry, i'm currently bad at rust and i just sorta brute forced this to work
fn get_all_files() -> Vec<VideoFile> {
    let mut all_files = fs::read_dir(".").unwrap()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    all_files.sort_by(|a, b| b.metadata().unwrap().modified().unwrap().cmp(&a.metadata().unwrap().modified().unwrap()));

    let stream_containers = all_files.iter()
        .map(|d| d.path())
        .map(|f| ffprobe_file(f.clone()))
        .filter(|p| p.is_some())
        .map(|f| f.unwrap())
        .collect::<Vec<_>>();

    return stream_containers;
}

fn do_file_picker(stream_containers: Vec<VideoFile>) -> Result<VideoFile, io::Error> {
    println!("Found ffmpeg friendly files:");
    for (index, entry) in stream_containers.iter().enumerate() {
        println!("{:>3}: {}", index, Path::new(&entry.path).file_name().unwrap().to_str().unwrap());
    }

    let target_index = input_helpers::get_number_input("Pick which file to remux: ");
    println!("You picked '{}'", target_index);

    return Ok(stream_containers.into_iter().nth(target_index as usize).unwrap());
}
