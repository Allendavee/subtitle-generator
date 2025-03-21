use hound;
use std::path::Path; // For reading WAV files
use std::process::Command;

// Converts timestamps to SRT format
pub fn format_timestamp(start_ms: i64, end_ms: i64) -> String {
    fn ms_to_time(ms: i64) -> String {
        let hours = ms / 3_600_000;
        let minutes = (ms % 3_600_000) / 60_000;
        let seconds = (ms % 60_000) / 1_000;
        let milliseconds = ms % 1_000;
        format!(
            "{:02}:{:02}:{:02},{:03}",
            hours, minutes, seconds, milliseconds
        )
    }

    format!("{} --> {}", ms_to_time(start_ms), ms_to_time(end_ms))
}

// Reads a WAV file and returns PCM samples
pub fn read_wav<P: AsRef<Path>>(path: P) -> Vec<f32> {
    let mut reader = hound::WavReader::open(path).expect("Failed to read WAV file");
    reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / 32768.0)
        .collect()
}

///CONVERTS THE AUDIO TO THE FORMAT THE MODEL CAN TRANSCRIBE
pub fn convert_audio(input: &str, output: &str) -> Result<(), std::io::Error> {
    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            input, // Input file
            "-ac",
            "1", // Convert to mono
            "-ar",
            "16000", // Set sample rate to 16kHz
            "-sample_fmt",
            "s16",  // Set sample format to 16-bit PCM
            output, // Output file
        ])
        .status()?;

    if status.success() {
        //println!("Audio conversion successful: {}", output);
        Ok(())
    } else {
        eprintln!("Audio conversion failed");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "FFmpeg failed",
        ))
    }
}
