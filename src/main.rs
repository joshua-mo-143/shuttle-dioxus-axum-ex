use axum::{routing::{get, post}, Form, Router, response::{Redirect, IntoResponse, Html}};
use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use tracing::warn;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::thread::Thread;

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

struct CustomService {}

#[shuttle_runtime::main]
async fn axum(
    // #[shuttle_shared_db::Postgres] db: PgPool
) -> Result<CustomService, shuttle_runtime::Error> {
    Ok(CustomService{})
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(
        mut self,
        addr: std::net::SocketAddr
    ) -> Result<(), shuttle_runtime::Error> {
        let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();

        thread::spawn(|| move {
            config.launch();
        });

        Ok(())
    }
}

#[derive(Clone, Routable, Debug, PartialEq, Deserialize, Serialize)]
enum Route {
    #[route("/")]
    Home {}
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            "Hello world!"
        }
    })
}