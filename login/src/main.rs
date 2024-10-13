use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {<App/>})
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
    view! {
    <button class="guest-login">Enter as Guest</button>
    }
}

#[component]
fn TypeSelection() -> impl IntoView {
    let length = 5;
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
