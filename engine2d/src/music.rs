use rodio::Sink;
use rodio::Source;
use std::time::Duration;

fn music() {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file1 = File::open("pigeonFly.mp3").unwrap();
    let file2 = File::open("pigeonCoo.mp3").unwrap();
    let file3 = File::open("backgroundNoise.mpe").unwrap();
    let source1 = rodio::Sink::try_new(&stream_handle).unwrap();
    let source2 = rodio::Sink::try_new(&stream_handle).unwrap();
    let source3 = rodio::Sink::try_new(&stream_handle).unwrap();

    source1.set_volume(1.75);
    source2.set_volume(1.0);
    source3.set_volume(.75);

    source1 = source1
        .take_duration(Duration::from_secs(5))
        .repeat_infinite();
    source2 = source2
        .take_duration(Duration::from_secs(45))
        .repeat_infinite();
    source3 = source3
        .take_duration(Duration::from_secs(180))
        .repeat_infinite();

    // stream_handle.play_raw(source.convert_samples());
}
