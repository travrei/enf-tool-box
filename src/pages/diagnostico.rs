use crate::model::model;
use dioxus::{prelude::*, web::WebEventExt};
use dioxus_logger::tracing::info;

use crate::system::system_diagnostico;

#[component]
pub fn Diagnostico() -> Element {
    let mut evolucao = use_signal(|| String::new());

    let mut submitted_prompt = use_signal(|| String::new());

    let mut text = use_resource(move || model(submitted_prompt().to_string()));

    let on_submit = move |event: FormEvent| {
        let web_event = event.web_event();
        web_event.prevent_default();

        let system_prompt = system_diagnostico();

        let prompt = format!("{}, {}", system_prompt, evolucao);

        info!("Enviando Prompt: {prompt}");

        submitted_prompt.set(prompt);
    };

    rsx! {
        div{class:"container",
            div{
                h1{"Diagnóstico de Enfermagem"}
                form{
                    h3{"Insira sua evolução!"}
                    input{name: "Evolucao",
                        placeholder:"Cole aqui sua evolução de Enfermagem",
                        value:"{evolucao}",
                        oninput:move |e| evolucao.set(e.value())
                    }
                    input{r#type: "submit"}
                }
            }
        }
    }
}
