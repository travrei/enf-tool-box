use crate::model::model;
use dioxus::{prelude::*, web::WebEventExt};
use dioxus_logger::tracing::info;

pub fn Gerador() -> Element {
    let mut prompt = use_signal(|| String::new());
    let mut submitted_prompt = use_signal(|| String::new());
    let mut text = use_resource(move || model(submitted_prompt().to_string()));

    let on_submit = move |event: FormEvent| {
        let web_event = event.web_event();
        web_event.prevent_default();
        info!("Enviando Prompt: {}", prompt());
        submitted_prompt.set(prompt().to_string());
    };
    if submitted_prompt().is_empty() {
        text.cancel();
    }

    rsx! {
        div { class: "container",
            form { onsubmit: on_submit,
                input {
                    name: "Prompt",
                    value: "{prompt}",
                    oninput: move |e| prompt.set(e.value())
                }
                input { r#type: "submit" }
            }

            div { class: "respostacont",
                h1 { class: "resposta_titulo", "Resultado" }

                match &*text.read(){
                    Some(Ok(result)) => rsx!{p{class:"resposta", "{result}"}},
                    Some(Err(e)) => rsx!{p{class:"resposta", "{e}"}},
                    None => rsx!{p{class:"resposta", "Sem Texto até o momento!"}}
                    }
            }
        }
    }
}
