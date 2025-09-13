use crate::audio::start_sound_rng;
use crate::styles::{
    BUTTON_STYLE, CARD_CONTAINER_STYLE, FORM_GROUP_STYLE, INPUT_STYLE, LABEL_STYLE,
    STATUS_ACTIVE_STYLE, STATUS_INACTIVE_STYLE, STOP_BUTTON_STYLE, TITLE_STYLE,
};

use leptos::prelude::*;
use rodio::Sink;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[component]
pub fn SoundPlayer(sound: &'static [u8], title: &'static str) -> impl IntoView {
    let (average_gap, set_average_gap) = signal(2.0);

    let (is_regular, set_is_regular) = signal(true);
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

        let effective_std_dev = if is_regular.get() {
            0.0
        } else {
            average_gap.get()
        };

        start_sound_rng(
            sink,
            sound,
            average_gap.get(),
            effective_std_dev,
            should_stop,
            set_is_active,
        );
    };

    let radio_name = format!("timing-mode-{}", title.replace(" ", "-").to_lowercase());
    let regular_id = format!("regular-radio-{}", title.replace(" ", "-").to_lowercase());
    let irregular_id = format!("irregular-radio-{}", title.replace(" ", "-").to_lowercase());

    view! {
        <div style=CARD_CONTAINER_STYLE>
            <h2 style=TITLE_STYLE>{title}</h2>

            <div style=FORM_GROUP_STYLE>
                <div style="display: flex; gap: 16px; align-items: center; flex-wrap: wrap;">
                    <div style="display: flex; align-items: center; gap: 8px;">
                        <label style=LABEL_STYLE>"Gap (s)"</label>
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

                        <input
                            type="radio"
                            id=regular_id.clone()
                            name=radio_name.clone()
                            prop:checked=move || is_regular.get()
                            on:change=move |_| {
                                set_is_regular.set(true);
                            }
                        />
                        <label for=regular_id style=LABEL_STYLE>
                            "Regular"
                        </label>

                        <input
                            type="radio"
                            id=irregular_id.clone()
                            name=radio_name
                            prop:checked=move || !is_regular.get()
                            on:change=move |_| {
                                set_is_regular.set(false);
                            }
                        />
                        <label for=irregular_id style=LABEL_STYLE>
                            "Irregular"
                        </label>
                    </div>
                </div>
            </div>

            <div style=move || if is_active.get() { STATUS_ACTIVE_STYLE } else { STATUS_INACTIVE_STYLE }>
                {move || if is_active.get() {
                    "🔊 Active"
                } else {
                    "🔇 Inactive"
                }}
            </div>

            {move || if is_active.get() {
                view! {
                    <button style=STOP_BUTTON_STYLE on:click=stop_sound>
                        "Stop"
                    </button>
                }.into_any()
            } else {
                view! {
                    <button style=BUTTON_STYLE on:click=start_sound>
                        "Start"
                    </button>
                }.into_any()
            }}
        </div>
    }
}
