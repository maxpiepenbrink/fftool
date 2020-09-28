use std::borrow::Borrow;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use serde_json::Value;

#[derive(Debug)]
pub struct VideoFile {
    pub path: String,
    pub streams: Vec<CodecStream>,
}

#[derive(Debug)]
pub struct CodecStream {
    pub codec_name: String,
    pub codec_type: String,
    pub tag_title: Option<String>,
    pub index: i32,
}

// take a path and run it through ffprobe, if we get real looking json back from ffprobe
// we can assume it contains valid streams for now
pub fn ffprobe_file<'a>(path: PathBuf) -> Option<VideoFile> {
    let path_str = path.to_str().unwrap();
    let output = Command::new("ffprobe")
        .args(&[
            path_str,
            "-show_streams",
            "-v",
            "quiet",
            "-print_format",
            "json",
        ])
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let output_string = str::from_utf8(&*output.stdout).unwrap();

        let js: Value = serde_json::from_str(output_string).unwrap();

        // simple validation
        if !js.is_object() {
            return None;
        }

        // this is optimistic, if we see anything unexpected we'll fail pretty badly
        let streams = match js["streams"].borrow() {
            Value::Array(objs) => {
                objs.iter().map(|e| {
                    let tag_title = e.get("tags")
                        .map_or(None, |obj| obj.get("title"))
                        .map_or(None, |obj| Option::from(obj.to_string()));
                    CodecStream {
                        index: e["index"].as_i64().unwrap() as i32,
                        codec_name: e["codec_name"].as_str().unwrap().to_string(),
                        codec_type: e["codec_type"].as_str().unwrap().to_string(),
                        tag_title,
                    }
                }).collect::<Vec<_>>()
            }
            _ => vec![]
        };

        // we only want valid video/audio containers
        let video_streams = streams.iter().filter(|f| f.codec_type == "video").count();
        let audio_streams = streams.iter().filter(|f| f.codec_type == "audio").count();
        if video_streams == 0 || audio_streams == 0 { return None; }

        return Some(VideoFile {
            path: path_str.to_string(),
            streams,
        });
    }

    return None;
}