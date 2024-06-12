use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_workers::worker;
use serde::{Serialize,Deserialize};

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// #[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct MyRequest;

// #[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyResponse;


// #[worker(leptos_workers::workers::ChannelWorker)]

#[worker(MyFutureWorker)]
pub async fn future_worker(_req: MyRequest) -> MyResponse {
    MyResponse
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);


    // in your component
    let response = create_local_resource(|| (), move |_| {
        future_worker(MyRequest)
    });

    eprintln!("Response is \n\n");
    dbg!(&response);

    view! {
        <h1>"Welcome to push notifications with  Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
