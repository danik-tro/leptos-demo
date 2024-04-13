mod components;

use components::cells::Cells;
use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! { <Login/> }
}

#[component]
fn Login() -> impl IntoView {
    view! {
        <section>
            <Cells/>
            <div class="signin">
                <div class="content">
                    <h2>Sign In</h2>
                    <div class="form">
                        <div class="inputBox">
                            <input type="text" required/>
                            <i>Username</i>
                        </div>
                        <div class="inputBox">
                            <input type="password" required/>
                            <i>Password</i>
                        </div>
                        <div class="links">
                            <a href="#">Forgot Password</a>
                            <a href="#">Signup</a>
                        </div>

                        <div class="inputBox">
                            <input type="submit" value="Login"/>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FirstComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            class:red=move || count.get() % 2 == 1
        >

            // on stable, this is move || count.get();

            {move || {
                if count.get() == 0 {
                    "Click me for the first time".to_string()
                } else {
                    format!("Click me again in the {} time!", count.get())
                }
            }}

        </button>

        <ProgressBar progress=count/>
    }
}

#[component]
fn ProgressBar(progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max="100"
            // now this works
            value=progress
        ></progress>
    }
}
