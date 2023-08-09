use axum::{routing::{get, post}, Form, Router, response::{Redirect, IntoResponse, Html}};
use serde::Deserialize;
use dioxus::prelude::*;
use tracing::warn;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

const APP_NAME: &'static str = "My App";

async fn submitted() -> Html<String> {
        Html(dioxus_ssr::render_lazy(rsx!{
        div {
            p {
                "You've submitted the form!"
            }
        }
    }))
}

#[derive(Props)]
pub struct ChildrenProps<'a> {
    title: String,
    children: Element<'a>
}

#[allow(non_snake_case)]
pub fn Layout<'a>(cx: Scope<'a, ChildrenProps<'a>>) -> Element {
	cx.render(rsx! {
		head {
            title {
                "{&*cx.props.title}",
            },
            meta {
                "charset": "utf-8",
            },
            meta {
                "name": "viewport",
                "content": "width=device-width, initial-scale=1.0"
            },
        }
        &cx.props.children
	})
}

#[derive(Deserialize)]
struct FormA {
    meme: String
}

async fn post_form(Form(form): Form<FormA>) -> impl IntoResponse {
    warn!("{}", form.meme);

    Redirect::to("/submitted")
}

async fn form() -> Html<String> {
        Html(dioxus_ssr::render_lazy(rsx!{
        Layout {
            title: format!("{APP_NAME} - Form"),
        form {
            "method": "POST",
            "action": "/formsubs",
            "name": "myForm",
            "prevent_default": "onsubmit",
            onsubmit: move |evt| println!("{evt:?}"),
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
    }}))
}

pub type AppState = Arc<Mutex<App>>;

pub struct App {
    db: PgPool,
}

#[shuttle_runtime::main]
async fn axum(
    // #[shuttle_shared_db::Postgres] db: PgPool
) -> shuttle_axum::ShuttleAxum {

    // let state = Arc::new(Mutex::new(App {db}));

    let router = Router::new()
        .route("/", get(form))
        .route("/submitted", get(submitted))
        .route("/formsubs", post(post_form));
        // .with_state(state);

    Ok(router.into())
}