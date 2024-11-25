use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
    <TypeSelection/>
    <GuestLogin/>
    <div class="container">
        <h2 class="titles">DISLEXIA</h2>
        <LoginForm/>
        <div class="links">
            <a href="#">Register</a> | <a href="#">Forgot Password?</a>
        </div>
    </div>
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    view! {

        Client-side rendering (CSR) with Trunk - a great option if you just want to make a snappy website with Leptos, or work with a pre-existing server or API. In CSR mode, Trunk compiles your Leptos app to WebAssembly (WASM) and runs it in the browser like a typical Javascript single-page app (SPA). The advantages of Leptos CSR include faster build times and a quicker iterative development cycle, as well as a simpler mental model and more options for deploying your app. CSR apps do come with some disadvantages: initial load times for your end users are slower compared to a server-side rendering approach, and the usual SEO challenges that come along with using a JS single-page app model apply to Leptos CSR apps as well. Also note that, under the hood, an auto-generated snippet of JS is used to load the Leptos WASM bundle, so JS must be enabled on the client device for your CSR app to display properly. As with a
        <form action="#" class="form" method="POST">
            <input type="email" class="input" placeholder="user@example.com" required/>
            <input type="password" class="input" placeholder="Password1234" required/>
            <button class="btn" type="submit">Login</button>
        </form>
    }
}

#[component]
fn GuestLogin() -> impl IntoView {
    view! {
    <button class="guest-login">Enter as Guest</button>
    }
}

#[component]
fn TypeSelection() -> impl IntoView {
    let length = 4;
    let counters = (1..=length).map(|idx| create_signal(idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        class:btn=move || count() % 2 == 0
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                    Button
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}
