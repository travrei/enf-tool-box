use crate::model::model;
use dioxus::{prelude::*, web::WebEventExt};
use dioxus_logger::tracing::info;

use crate::system::system_diagnostico;

pub fn Diagnostico() -> Element {
    let mut evolucao = use_signal(|| String::new());

    let mut submitted_prompt = use_signal(|| String::new());

    let mut model = use_resource(move || model(submitted_prompt().to_string()));

    let on_submit = move |event: FormEvent| {
        let web_event = event.web_event();
        web_event.prevent_default();

        let system_prompt = system_diagnostico();

        let prompt = format!("{}, {}", system_prompt, evolucao);

        info!("Enviando Prompt: {prompt}");

        submitted_prompt.set(prompt);
    };

    if submitted_prompt().is_empty(){model.cancel();}

    rsx! {
        div{class:"container",
            div{
                h1{"Diagnóstico de Enfermagem"}
                form{ onsubmit: on_submit,
                    h3{"Insira sua evolução!"}
                    input{name: "Evolucao",
                        placeholder:"Cole aqui sua evolução de Enfermagem",
                        value:"{evolucao}",
                        oninput: move |e| evolucao.set(e.value())
                    }
                    input{r#type: "submit"}
                }

                div { class: "respostacont",
                    h4 { class: "resposta_titulo", "Resultado" }

                    match &*model.read(){
                        Some(Ok(result)) => {let resposta = result.to_string(); rsx!{p{class:"resposta", dangerous_inner_html: "{resposta}"}}},
                        Some(Err(e)) => {info!("{e}"); rsx!{p{class:"resposta", "{e}"}}},
                        None => rsx!{p{class:"resposta", "Sem Texto até o momento!"}}
                    }
                }
            }
        }
    }
}
