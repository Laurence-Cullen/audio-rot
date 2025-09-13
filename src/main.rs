use leptos::mount::mount_to_body;
use leptos::prelude::*;
use rodio::Source;
// use rodio::source::noise::Velvet;
use std::io::BufReader;
// use std::time::Duration;

const PIPE_SOUND: &[u8] = include_bytes!("../samples/metal-pipe-falling.mp3");

fn main() {
    mount_to_body(App);
}

fn play_sound() {
    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.
    let stream =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(std::io::Cursor::new(PIPE_SOUND));

    let source = rodio::Decoder::try_from(file).unwrap().buffered();

    let original_source = source.clone();

    let sink = rodio::Sink::connect_new(stream.mixer());
    sink.append(original_source);
    sink.play();
    // // Keep sleeping until the sink is empty
    while !sink.empty() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[component]
fn App() -> impl IntoView {
    let (num_reader, num_writer) = signal(0);

    view! {
        <button
            on:click=move |_| {
                num_writer.set(num_reader.get() + 1);
                play_sound();
            }
        >
            "Click me: "
            {num_reader}
        </button>
        <p>
            "Double count: "
            {move || num_reader.get() * 3}
        </p>
    }
}
