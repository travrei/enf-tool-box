use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        nav {}
        div { class: "container",
            div {
                h1 { class: "title", "Enf Tool Box!!" }
                h4 { class: "secondary_title", "Uma Caixa de Ferramentas Para enfermagem!" }
            }
            div { class: "items",
                Link { to: Route::Braden {},
                    a { class: "button", "Braden" }
                }
                Link { to: Route::Braden {},
                    a { class: "button", "Morse" }
                }
                Link { to: Route::Braden {},
                    a { class: "button", "Fugulin" }
                }
                Link { to: Route::Braden {},
                    a { class: "button", "Glasgow" }
                }
                Link { to: Route::Gerador {},
                    a { class: "button", "Gerador de Evolução" }
                }
            }
        }
    }
}
