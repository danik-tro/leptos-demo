use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count.get() % 2 == 1
        >

            // on stable, this is move || count.get();
            "Click me: "
            {move || count.get()}
        </button>
    }
}
