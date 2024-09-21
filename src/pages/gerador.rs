use dioxus::prelude::*;

use crate::gemini::generate_gemini;

pub fn Gerador() -> Element {
    let gemini = use_future(generate_gemini);
    let text = gemini.to_owned();
    rsx! {
        p{"{text}"}
    }
}
