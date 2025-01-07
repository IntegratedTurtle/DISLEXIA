use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
use wasm_bindgen::{convert::FromWasmAbi, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{console, window, RequestInit, Response, Window};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {<App/>})
}

fn get_storage(window: &Window, field: &str) -> Option<String> {
    if let Some(storage) = window.local_storage().ok().flatten() {
        if let Ok(Some(data)) = storage.get_item(field) {
            return Some(data);
        }
    }
    return None;
}

fn set_storage(window: &Window, field_name: &str, data: &str) {
    if let Some(storage) = window.local_storage().ok().flatten() {
        if let Err(e) = storage.set_item(field_name, data) {
            console_error(&format!("Failed to set localStorage: {:?}", e));
        }
    } else {
        console_error(&"localStorage is not available.");
    }
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
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
        <form action="#" class="form" method="POST">
            <input type="email" class="input" placeholder="user@example.com" required/>
            <input type="password" class="input" placeholder="Password1234" required/>
            <button class="btn" type="submit">Login</button>
        </form>
    }
}

#[component]
fn GuestLogin() -> impl IntoView {
    if let Some(window) = window() {
        console_log(&format!(
            "Guest from localStorage: {:?}",
            get_storage(&window, "guest")
        ));
        console_log(&format!(
            "Data from localStorage: {:?}",
            get_storage(&window, "data")
        ));
    }

    let make_request = move || {
        spawn_local(async move {
            let request = RequestInit::new();
            request.set_method("SEARCH"); // Use POST for sending a body
            request.set_body(&JsValue::from_str(
                "{\"email\": \"user@example.com\",\"password\": \"securepassword\"}",
            ));

            if let Some(window) = window() {
                let fetch_promise =
                    window.fetch_with_str_and_init("http://127.0.0.1:8081", &request);

                match JsFuture::from(fetch_promise).await {
                    Ok(response_value) => {
                        let response: web_sys::Response = response_value.dyn_into().unwrap();

                        match JsFuture::from(response.text().unwrap()).await {
                            Ok(text) => {
                                let response_text: String = text.as_string().unwrap();
                                set_storage(&window, "guest", "true");
                                set_storage(&window, "data", &response_text);
                                let _ = window.location().set_href("/main");
                            }
                            Err(err) => unsafe {
                                console::log_1(
                                    &format!("Error reading response body: {:?}", err).into(),
                                );
                            },
                        }
                    }
                    Err(err) => unsafe {
                        console::log_1(&format!("Error fetching data: {:?}", err).into());
                    },
                }
            }
        });
    };

    view! {
        <button
            on:click= move |_| make_request()
            class="guest-login">
            "Enter as Guest"
        </button>
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
