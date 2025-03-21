# Rust Subtitle Generator
A fast and efficient subtitle generator built with Rust.
This tool scans video/audio files, detects speech using whisper_rs, and generates .srt subtitles with high accuracy and speed.
Features include single-language support (English only for now), parallel processing, and robust error handling.

I recently picked up Rust. After learning so much, I decided to build this project out of silent frustration.
I love watching movies with subtitles, I've been following Suits LA but each new episode doesn't come with subtitles
so I decided to build this since I am learning how to program in Rust. Though I have a little bit of experience in C, it wasn't so hard learning some Rust concepts.
-----------------------------------------------------------------------------------------------------------------------------
# Key Notes
* Ensure your dependencies are up to date
* Ensure to install ffmpeg and Whisper-rs (You can use brew to do this on MacOS)
* Download and add the whisper model to the "models" folder (curl -L -H "Hugging-Secrete" -o ggml-base.en.bin https://huggingface.co/openai/whisper/resolve/main/ggml-base.en.bin)

# How To Use
* The program would prompt you to add the path to the audio file
* It would ask you for the name you want to give the srt file (include ".srt" extension to your file name)
# ____________________________________________________
# Testing and Output
* Check the subtitle.srt file to see the transcription of this audio (https://drive.google.com/file/d/1Raz0vmhNKbcsaCtg9r-6WeDdg9TWdMyI/view?usp=sharing)

# Challenges Faced 🤣🤣
* The model was transcribing gibberish initially, I thought maybe I wasn't using the right model or I needed to pay to use the advanced trained model
* I had to go back to the Whisper documentation to read up
* I found out the audio has to be converted to a particular format using ffmpeg (check the convert_audio function)
* I am still learning how to be a better programmer in Rust but I am excited I could pull this off
* Thanks to the Rust compiler debugging wasn't so hard

# What Next
* I'll keep learning and looking at improving this project to handle other languages
