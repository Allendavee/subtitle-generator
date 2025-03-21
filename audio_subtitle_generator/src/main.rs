use std::fs;
use std::fs::File;
use std::io::Write;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};
mod utils;

fn main() {
    // Whisper model path
    let model_path = "models/ggml-base.en.bin";
    let audio_path = "audio/audio.wav";
    println!("Enter Audio Path");
    let mut audio_path_cl = String::new();
    //Path to get audio file
    std::io::stdin()
        .read_line(&mut audio_path_cl)
        .expect("Failed to read line");
    // Path to extracted audio converted to string slice
    let temp_audio_path = audio_path_cl.trim();
    // Step 1: Convert audio
    if let Err(e) = utils::convert_audio(temp_audio_path, audio_path) {
        eprintln!("Conversion failed: {}", e);
        return;
    }

    // Check if file exists before processing
    if fs::metadata(audio_path).is_err() {
        eprintln!("Error: File '{}' not found.", audio_path);
        return;
    }
    //Naming the subtitle output file from user input
    let mut subtitle_name = String::new();
    println!("Enter Subtitle Name: ");
    std::io::stdin()
        .read_line(&mut subtitle_name)
        .expect("Failed to read line");

    let output_srt = subtitle_name.trim();

    // Load Whisper model
    let ctx = WhisperContext::new_with_params(model_path, Default::default())
        .expect("Failed to load Whisper model");
    let mut state = ctx.create_state().expect("Failed to create Whisper state");
    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // Read and preprocess audio
    let audio_samples = utils::read_wav(audio_path);

    // Run transcription
    state
        .full(params, &audio_samples)
        .expect("Failed to transcribe file");

    // Generate subtitles
    let mut srt_output = File::create(output_srt).expect("Failed to create subtitle file");

    let num_segments = state.full_n_segments();
    for i in 0..num_segments.expect("Error getting number of segments") {
        let segment = state.full_get_segment_text(i).unwrap();
        let start_time = state.full_get_segment_t0(i).unwrap() * 10; // Convert to ms
        let end_time = state.full_get_segment_t1(i).unwrap() * 10; // Convert to ms
        let timestamp = utils::format_timestamp(start_time, end_time);

        writeln!(srt_output, "{}\n{}\n{}\n", i + 1, timestamp, segment)
            .expect("Failed to write to subtitle file");
    }

    println!("Subtitles saved to {}", output_srt);
}
