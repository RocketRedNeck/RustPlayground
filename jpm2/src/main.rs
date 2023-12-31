use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};  // https://docs.rs/rodio/latest/rodio/

fn main() {

    
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open("fitz.mp3").unwrap();
    let reader = BufReader::new(file);

    // Decode that sound file into a source
    let source = Decoder::new_mp3(reader).unwrap();

    // Play the sound directly on the device
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();

}
