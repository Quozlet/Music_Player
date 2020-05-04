use dirs;
use rodio;
use std::io::{prelude::Write, BufReader};
use tinyfiledialogs;
use walkdir::WalkDir;

fn main() {
    print!("[1] Play multiple songs\n[2] Play whole directory\n[3] Exit\n[1]/[2]/[3]: ");
    std::io::stdout().flush().unwrap();
    let mut user_input = std::string::String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    if &user_input[0..1] == "1" {
        play_many();
    } else if &user_input[0..1] == "2" {
        play_dir();
    } else if &user_input[0..1] == "3" {
        println!("Exiting...");
        std::process::exit(0);
    }
}

fn ask_for_exit() {
    println!("Exit? [Y/N]");
    std::io::stdin().lock();
    let mut user_response = std::string::String::new();
    std::io::stdin().read_line(&mut user_response).unwrap();
    if &user_response[0..1].to_lowercase() == "y" {
        println!("Exiting...");
        std::process::exit(0);
    }
}

fn play_many() {
    let music_files = match tinyfiledialogs::open_file_dialog_multi("Choose your songs", dirs::audio_dir().unwrap().to_str().unwrap(), Some((&["mp3"], "audio"))) {
        Some(files) => files,
        _ => return ask_for_exit(),
    };
    let device: rodio::Device = rodio::default_output_device().unwrap();
    let sink: rodio::Sink = rodio::Sink::new(&device);
    let _: Vec<_> = music_files
        .iter()
        .map(|song: &std::string::String| {
            println!("Playing '{}'", std::path::Path::new(&song).file_stem().and_then(std::ffi::OsStr::to_str).unwrap());
            let file = std::fs::File::open(&song).unwrap();
            sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
            sink.sleep_until_end();
        })
        .collect();
}

fn play_dir() {
    let music_dir = match tinyfiledialogs::select_folder_dialog("Choose your directory", dirs::audio_dir().unwrap().to_str().unwrap()) {
        Some(files) => files,
        _ => return ask_for_exit(),
    };
    let device: rodio::Device = rodio::default_output_device().unwrap();
    let sink: rodio::Sink = rodio::Sink::new(&device);
    let walker = WalkDir::new(music_dir).follow_links(true).into_iter();
    for song in walker.filter_entry(|dir| !dir.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)) {
        if let Ok(file) = song {
            if let Ok(metadata) = file.metadata() {
                if !metadata.is_dir() {
                    println!("Playing {}", file.file_name().to_owned().into_string().unwrap());
                    let file = std::fs::File::open(file.path()).unwrap();
                    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                    sink.sleep_until_end();
                } else {
                    println!("Decending into directory {}", file.file_name().to_owned().into_string().unwrap());
                }
            }
        }
    }
}
