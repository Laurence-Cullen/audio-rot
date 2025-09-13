mod styles;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use rand::rng;
use rand_distr::{Distribution, Normal};
use rodio::{Sink, Source};
use std::io::BufReader;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::styles::{
    BUTTON_STYLE, FORM_CONTAINER_STYLE, INPUT_STYLE, LABEL_STYLE, LABEL_TEXT_STYLE,
    MAIN_CONTAINER_STYLE, STATUS_ACTIVE_STYLE, STATUS_INACTIVE_STYLE, STOP_BUTTON_STYLE,
    TITLE_STYLE,
};

const PIPE_SOUND: &[u8] = include_bytes!("../samples/metal-pipe-falling.mp3");
const DISCORD_NOTIFICATION: &[u8] = include_bytes!("../samples/discord-notification.mp3");

fn main() {
    mount_to_body(|| view! { <App sound=DISCORD_NOTIFICATION title="Discord notification" /> });
}

fn sample_normal(mean: f32, std_dev: f32) -> f32 {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rng();
    normal.sample(&mut rng)
}

fn start_sound_rng(
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

#[component]
fn App(sound: &'static [u8], title: &'static str) -> impl IntoView {
    let (average_gap, set_average_gap) = signal(2.0);
    let (std_dev, set_std_dev) = signal(0.5);
    let (is_active, set_is_active) = signal(false);
    let (audio_sink, set_audio_sink) = signal(None::<Arc<Sink>>);
    let (stop_flag, set_stop_flag) = signal(None::<Arc<AtomicBool>>);

    let stop_sound = move |_| {
        if let Some(sink) = audio_sink.get() {
            sink.stop();
        }
        if let Some(flag) = stop_flag.get() {
            flag.store(true, Ordering::Relaxed);
        }
        set_is_active.set(false);
        set_audio_sink.set(None);
        set_stop_flag.set(None);
    };

    let start_sound = move |_| {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Failed to open default audio stream");
        let sink = Arc::new(rodio::Sink::connect_new(stream.mixer()));
        let should_stop = Arc::new(AtomicBool::new(false));

        set_stop_flag.set(Some(should_stop.clone()));
        set_audio_sink.set(Some(sink.clone()));

        start_sound_rng(
            sink,
            sound,
            average_gap.get(),
            std_dev.get(),
            should_stop,
            set_is_active,
        );
    };

    view! {
        <div style=MAIN_CONTAINER_STYLE>
            <div style=FORM_CONTAINER_STYLE>
                <h2 style=TITLE_STYLE>{title}</h2>

                <label style=LABEL_STYLE>
                    <span style=LABEL_TEXT_STYLE>"Average Gap (seconds)"</span>
                    <input
                        type="number"
                        step="0.1"
                        style=INPUT_STYLE
                        prop:value=move || average_gap.get().to_string()
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                set_average_gap.set(val);
                            }
                        }
                    />
                </label>

                <label style=LABEL_STYLE>
                    <span style=LABEL_TEXT_STYLE>"Standard Deviation"</span>
                    <input
                        type="number"
                        step="0.1"
                        style=INPUT_STYLE
                        prop:value=move || std_dev.get().to_string()
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                set_std_dev.set(val);
                            }
                        }
                    />
                </label>

                <div style=move || if is_active.get() { STATUS_ACTIVE_STYLE } else { STATUS_INACTIVE_STYLE }>
                    {move || if is_active.get() {
                        "🔊 Sound Generator Active"
                    } else {
                        "🔇 Sound Generator Inactive"
                    }}
                </div>

                {move || if is_active.get() {
                    view! {
                        <button style=STOP_BUTTON_STYLE on:click=stop_sound>
                            "Stop Sound"
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <button style=BUTTON_STYLE on:click=start_sound>
                            "Start Sound"
                        </button>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
