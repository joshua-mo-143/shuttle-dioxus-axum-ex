use axum::{routing::{get, post}, Form, Router, response::{Redirect, IntoResponse, Html}};
use serde::Deserialize;
use dioxus::prelude::*;
use tracing::warn;

async fn hello_world() -> Html<String> {
    // create a VirtualDom with the app component
    let mut app = VirtualDom::new(page);
    // rebuild the VirtualDom before rendering
    let _ = app.rebuild();

    Html(dioxus_ssr::render(&app))

}

#[derive(Deserialize)]
struct FormA {
    meme: String
}

async fn post_form(Form(form): Form<FormA>) -> impl IntoResponse {
    warn!("{}", form.meme);

    axum::http::StatusCode::OK
}

async fn form() -> Html<String> {

    let onsubmit = move |evt: FormEvent| {
        cx.spawn(async move {
            let resp = reqwest::Client::new()
                .post("http://localhost:8000/post_form")
                .form(&[
                    ("meme", &evt.values["meme"]),
                ])
                .send()
                .await;

            match resp {
                // Parse data from here, such as storing a response token
                Ok(_data) => println!("Login successful!"),

                //Handle any errors from the fetch here
                Err(_err) => {
                    println!("Login failed - you need a login server running on localhost:8080.")
                }
            }
        });
    };
    
        Html(dioxus_ssr::render_lazy(rsx!{
        form {
            "method": "POST",
            "action": "/formsubs",
            "name": "myForm",
            "prevent_default": true,
            onsubmit: onsubmit,
            "Meme",
            label {
                "for": "meme",
                span {
                    "Meme"
                },
                input {
                    "name": "meme",
                    "placeholder": "Put your meme in here."
                },
            },
            button {
                "type": "submit",
                "Submit"
            }
        }
    }))
}


#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/form", get(form))
        .route("/formsubs", post(post_form));

    Ok(router.into())
}

fn page(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            "Hello world!"
        }
    })
}