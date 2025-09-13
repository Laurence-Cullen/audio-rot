mod audio;
mod player_component;
mod styles;

use leptos::mount::mount_to_body;
use leptos::prelude::*;

use crate::player_component::SoundPlayer;
use crate::styles::MAIN_CONTAINER_STYLE;

fn main() {
    mount_to_body(|| view! { <MainApp /> });
}

#[component]
fn MainApp() -> impl IntoView {
    view! {
        <div style=MAIN_CONTAINER_STYLE>
            <div style="display: flex; flex-wrap: wrap; gap: 24px; justify-content: center; align-items: flex-start;">
                <SoundPlayer sound=include_bytes!("../samples/discord-notification.mp3") title="Discord Notification" />
                <SoundPlayer sound=include_bytes!("../samples/metal-pipe-falling.mp3") title="Metal Pipe Falling" />
                <SoundPlayer sound=include_bytes!("../samples/most-annoying-sound-in-the-world.mp3") title="Most annoying sound in the world" />
                <SoundPlayer sound=include_bytes!("../samples/bing-chilling.mp3") title="Bing Chilling" />
                <SoundPlayer sound=include_bytes!("../samples/smoke-detector-beep.mp3") title="Smoke Detector" />
                <SoundPlayer sound=include_bytes!("../samples/screaming-emoji-meme.mp3") title="Screaming Emoji" />
                <SoundPlayer sound=include_bytes!("../samples/groan-tube.mp3") title="Groan tube" />
                <SoundPlayer sound=include_bytes!("../samples/air-horn.mp3") title="Air horn" />
            </div>
        </div>
    }
}
