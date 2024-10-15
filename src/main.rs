#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use pages::braden::Braden;
use pages::diagnostico::Diagnostico;
use pages::fugulin::Fugulin;
use pages::gerador::Gerador;
use pages::glasgow::Glasgow;
use pages::home::Home;
use pages::morse::Morse;

mod model;
mod pages;
mod system;

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
    #[route("/glasgow")]
    Glasgow {},
    #[route("/gerador")]
    Gerador {},
    #[route("/diagnostico")]
    Diagnostico {},
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
        link { rel: "manifest", href: "manifest.json" }
        script { src: "sw.js" }
        Router::<Route> {}
    }
}
