use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
use thiserror::Error;
use wasm_bindgen::{convert::FromWasmAbi, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, RequestInit, Response, Window};

const SERVER: &str = "http://127.0.0.1:8081";

#[derive(Debug, Error)]
enum RequestError {
    #[error("Window object not found")]
    WindowNotFound,
    #[error("Failed to fetch resource: {0:?}")]
    FetchError(JsValue),
    #[error("Failed to parse response as text: {0:?}")]
    ResponseParseError(JsValue),
    #[error("HTTP error: status code {0}")]
    HttpError(u16),
}

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

async fn make_request(
    url: String,
    method: String,
    body: String,
) -> Result<Option<String>, RequestError> {
    let mut request = RequestInit::new();
    request.set_method(&method);
    request.set_body(&JsValue::from_str(&body));

    if let Some(window) = web_sys::window() {
        let fetch_promise = window.fetch_with_str_and_init(&url, &request);

        // Wait for the fetch to complete
        match JsFuture::from(fetch_promise).await {
            Ok(response_value) => {
                let response: Response = response_value.dyn_into().unwrap();

                // Check if the response is okay
                if response.ok() {
                    match JsFuture::from(response.text().unwrap()).await {
                        Ok(text) => Ok(text.as_string()),
                        Err(err) => Err(RequestError::ResponseParseError(err)),
                    }
                } else {
                    Err(RequestError::HttpError(response.status()))
                }
            }
            Err(err) => Err(RequestError::FetchError(err)),
        }
    } else {
        Err(RequestError::WindowNotFound)
    }
}

#[component]
fn App() -> impl IntoView {
    let current_view = move || {
        let pathname = window()
            .expect("COuld not find window")
            .location()
            .pathname()
            .expect("could not access pathname");
        match pathname.as_str() {
            "/" => view! { <Login/> },
            _ => view! { <Default/> },
        }
    };

    view! {
        <main>
            {current_view}
        </main>
    }
}

#[component]
fn Default() -> impl IntoView {
    view! {
        <div class="container">
            <h2 class="titles">DISLEXIA</h2>
            <p>We are sorry but this site is currently not working</p>
        </div>
    }
}

#[component]
fn Login() -> impl IntoView {
    if let Some(window) = web_sys::window() {
        match get_storage(&window, "data") {
            Some(data) => {
                if Some(get_storage(&window, "guest")) == Some(Some("true".to_string())) {
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
                } else {
                    let _ = window.location().set_href("/main");
                    console_log("is logged in");
                    view! {
                        <>
                        <Default/>
                        </>
                    }
                }
            }
            None => view! {
            <GuestLogin/>
            <div class="container">
                <h2 class="titles">DISLEXIA</h2>
                <LoginForm/>
                <div class="links">
                    <a href="#">Register</a> | <a href="#">Forgot Password?</a>
                </div>
            </div>
            },
        }
    } else {
        console_error("No Window");
        view! {<><Default/></>}
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (passwd, set_passwd) = create_signal("".to_string());
    let (error_message, set_error_message) = create_signal(None::<String>); // Signal to hold error messages

    let on_login = move || {
        let body = format!(
            "{{\"email\": \"{}\",\"password\": \"{}\"}}",
            email.get(),
            passwd.get()
        );
        console_log("Sending login request");
        console_log(&body);
        spawn_local(async move {
            match make_request(SERVER.to_string(), "SEARCH".to_string(), body).await {
                Ok(Some(response_text)) => {
                    if let Some(window) = web_sys::window() {
                        set_storage(&window, "guest", "false");
                        set_storage(&window, "data", &response_text);
                        console_log(&format!("Response: {}", response_text));
                        let _ = window.location().set_href("/main");
                    }
                }
                Ok(None) => {
                    console_log("Empty response body");
                }
                Err(RequestError::HttpError(val)) if val == 401 => {
                    console_log("Wrong password or username");
                    set_error_message.set(Some("Incorrect Password or Email".to_string()));
                }
                Err(err) => {
                    console_log(&format!("Request failed: {:?}", err));
                    set_error_message.set(Some("Unexpected error occurred".to_string()));
                }
            }
        });
    };

    view! {
        <form action="#" class="form">
            <input
                type="email"
                class="input"
                placeholder="user@example.com"
                value=email()   // Bind the value of the signal
                on:input=move |ev| set_email(event_target_value(&ev)) // Update the signal on input
                required
            />
            <input
                type="password"
                class="input"
                placeholder="Password1234"
                value=passwd()  // Bind the value of the signal
                on:input=move |ev| set_passwd(event_target_value(&ev)) // Update the signal on input
                required
            />
            {move || {
                    if let message = Some(error_message.get()) {
                        view!{
                            <div class="alert">
                                {message}
                            </div>
                        }
                    } else {
                        view! {<div></div>}
                    }
                }
            }
            <button
                on:click= move |_| on_login()
                class="btn"
                type="submit"
                >Login</button>
        </form>
    }
}

#[component]
fn GuestLogin() -> impl IntoView {
    let on_guest_login = move || {
        spawn_local(async move {
            match make_request(
                SERVER.to_string(),
                "SEARCH".to_string(),
                "{\"email\": \"user@example.com\",\"password\": \"securepassword\"}".to_string(),
            )
            .await
            {
                Ok(Some(response_text)) => {
                    if let Some(window) = web_sys::window() {
                        set_storage(&window, "guest", "true");
                        set_storage(&window, "data", &response_text);
                        console_log(&format!("Response: {}", response_text));
                        let _ = window.location().set_href("/main");
                    }
                }
                Ok(None) => {
                    console_log("Empty response body");
                }
                Err(err) => {
                    console_log(&format!("Request failed: {:?}", err));
                }
            }
        });
    };

    view! {
        <button
            on:click= move |_| on_guest_login()
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
