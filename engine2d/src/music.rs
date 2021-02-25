use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn music() {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file1 = File::open("bensound-energy.mp3").unwrap();
    //let file2 = File::open("pigeonCoo.mp3").unwrap();
    //let file3 = File::open("backgroundNoise.mp3").unwrap();
    let source1 = rodio::Decoder::new(BufReader::new(file1)).unwrap();
    //let source2 = rodio::Decoder::new(BufReader::new(file2)).unwrap();
    //let source3 = rodio::Decoder::new(BufReader::new(file3)).unwrap();

    let source1 = source1
        .take_duration(Duration::from_secs(5))
        .repeat_infinite();
    //let _source2 = source2
       // .take_duration(Duration::from_secs(45))
        //.repeat_infinite();
    //let _source3 = source3
        //.take_duration(Duration::from_secs(180))
        //.repeat_infinite();

    stream_handle.play_raw(source1.convert_samples());
    //stream_handle.play_raw(_source2.convert_samples());
    //stream_handle.play_raw(_source3.convert_samples());


}
