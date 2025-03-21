# Rust Subtitle Generator
A fast and efficient subtitle generator built with Rust.
This tool scans video/audio files, detects speech using whisper_rs, and generates .srt subtitles with high accuracy and speed.
Features include single-language support (English only for now), parallel processing, and robust error handling.
-----------------------------------------------------------------------------------------------------------------------------
# Key Notes
* Ensure your dependencies are up to date
* Ensure to install ffmpeg and Whisper-rs (You can use brew to do this on MacOS)
* Download and add the whisper model to the models folder (curl -L -H "Hugging-Secrete" -o ggml-base.en.bin https://huggingface.co/openai/whisper/resolve/main/ggml-base.en.bin)

# How To Use
* The program would prompt you to add the path to the audio file
* It would ask you for the name you want to give the srt file (include ".srt" extension to your file name)
# ___________________________________________________________________________________________________________________________
# Testing and Output
* Check the subtitle.srt file to see the transcription of this audio (https://drive.google.com/file/d/1Raz0vmhNKbcsaCtg9r-6WeDdg9TWdMyI/view?usp=sharing)
