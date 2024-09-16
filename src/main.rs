#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use dioxus_router::routable;
use pages::braden::Braden;
use pages::fugulin::Fugulin;
use pages::gerador::Gerador;
use pages::home::Home;
use pages::morse::Morse;

mod pages;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/braden")]
    Braden {},
    #[route("/morse")]
    Morse {},
    #[route("/fugulin")]
    Fugulin {},
    #[route("/gerador")]
    Gerador {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        head {
            meta {
                name: "viewport",
                content: "width-device-width, initial-scale=1.0"
            }
        }
        link { rel: "stylesheet", href: "main.css" }
        Router::<Route> {}
    }
}
