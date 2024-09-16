use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container",
            div {
                h1 { class: "title", "Enf Tool Box!!" }
                h4 { class: "secondary_title", "Uma Caixa de Ferramentas Para enfermagem!" }
            }
            div { class: "items",
                Link { to: Route::Braden {}, class: "button", "Braden" }
                Link { to: Route::Morse {}, class: "button", "Morse" }
                Link { to: Route::Fugulin {}, class: "button", "Fugulin" }
                Link { to: Route::Braden {}, class: "button", "Glasgow" }
                Link { to: Route::Gerador {}, class: "button", "Gerador de Evolução" }
            }
        }
    }
}
