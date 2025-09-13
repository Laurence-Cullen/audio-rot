use leptos::prelude::*;
use rand::rng;
use rand_distr::{Distribution, Normal};
use rodio::{Sink, Source};
use std::io::BufReader;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

fn sample_normal(mean: f32, std_dev: f32) -> f32 {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rng();
    normal.sample(&mut rng)
}

pub fn start_sound_rng(
    sink: Arc<Sink>,
    sound: &'static [u8],
    average_gap: f32,
    std_dev: f32,
    should_stop: Arc<AtomicBool>,
    set_is_active: WriteSignal<bool>,
) {
    sink.play();
    set_is_active.set(true);

    for _ in 0..100 {
        if should_stop.load(Ordering::Relaxed) {
            break;
        }

        let source = rodio::Decoder::try_from(BufReader::new(std::io::Cursor::new(sound.to_vec())))
            .expect("Failed to decode audio")
            .buffered();
        sink.append(source);

        let silence_duration = sample_normal(average_gap, std_dev).max(0.1);
        let silence = rodio::source::Zero::new(2, 44100)
            .take_duration(Duration::from_secs_f32(silence_duration));
        sink.append(silence);
    }

    while !sink.empty() {
        if should_stop.load(Ordering::Relaxed) {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    sink.stop();
    set_is_active.set(false);
}
