/// For reading WAV files
use hound;
use std::path::Path; 

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
