use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (num_reader, num_writer) = signal(0);

    view! {
        <button
            on:click=move |_| num_writer.set(num_reader.get() + 1)
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
