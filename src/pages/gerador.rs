use crate::model::model;
use crate::system::system_prompt;
use dioxus::{prelude::*, web::WebEventExt};
use dioxus_logger::tracing::info;

pub fn Gerador() -> Element {
    //SSVV
    let mut pa = use_signal(|| String::new());
    let mut fr = use_signal(|| String::new());
    let mut fc = use_signal(|| String::new());
    let mut temp = use_signal(|| String::new());
    let mut spo2 = use_signal(|| String::new());

    //Basico de Ev.
    let mut estgeral = use_signal(|| String::new());
    let mut queixas = use_signal(|| String::new());
    let mut torax = use_signal(|| String::new());
    let mut abdome = use_signal(|| String::new());
    let mut membros = use_signal(|| String::new());
    let mut eliminacoes = use_signal(|| String::new());
    let mut lesoes = use_signal(|| String::new());

    let mut diversos = use_signal(|| String::new());

    let mut submitted_prompt = use_signal(|| String::new());
    let mut text = use_resource(move || model(submitted_prompt().to_string()));

    let on_submit = move |event: FormEvent| {
        let web_event = event.web_event();
        web_event.prevent_default();

        let system_prompt = system_prompt();

        let prompt = format!(
            "{} Estado geral: {}, Queixas: {}, Torax: {}, Abdome: {}, Membros: {}, Eliminações: {}, Lesões: {}.
            SSVV: {}mmhg; {}irpm; {}bpm; {}ºC, {}%; Outras informações: {}",
            system_prompt,
            estgeral(),
            queixas(),
            torax(),
            abdome(),
            membros(),
            eliminacoes(),
            lesoes(),
            pa(),
            fr(),
            fc(),
            temp(),
            spo2(),
            diversos()
        );

        info!("Enviando Prompt: {}", prompt);
        submitted_prompt.set(prompt.to_string());
    };
    if submitted_prompt().is_empty() {
        text.cancel();
    }

    rsx! {
        div { class: "container",
            form {onsubmit: on_submit,
                div { class: "promtcont",
                    h3 { "Informações Gerais" }
                    input {
                        class: "prompt ",
                        name: "EstGeral",
                        placeholder: "Estado Geral",
                        value: "{estgeral}",
                        oninput: move |e| estgeral.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Queixas",
                        placeholder: "Queixas",
                        value: "{queixas}",
                        oninput: move |e| queixas.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Torax",
                        placeholder: "Torax",
                        value: "{torax}",
                        oninput: move |e| torax.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Abdome",
                        placeholder: "Abdome",
                        value: "{abdome}",
                        oninput: move |e| abdome.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Membros",
                        placeholder: "Membros",
                        value: "{membros}",
                        oninput: move |e| membros.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Eliminacoes",
                        placeholder: "Eliminações",
                        value: "{eliminacoes}",
                        oninput: move |e| eliminacoes.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Lesoes",
                        placeholder: "Lesões",
                        value: "{lesoes}",
                        oninput: move |e| lesoes.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Diversos",
                        placeholder: "Outras Informações",
                        value: "{diversos}",
                        oninput: move |e| diversos.set(e.value())
                    }
                }
                div { class: "promtcont",
                    h3 { "Sinais Vitais" }
                    input {
                        class: "prompt",
                        name: "PA",
                        placeholder: "Pressão Arterial",
                        value: "{pa}",
                        oninput: move |e| pa.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "FR",
                        placeholder: "Frequência Respiratória",
                        value: "{fr}",
                        oninput: move |e| fr.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "FC",
                        placeholder: "Frequência Cardíaca",
                        value: "{fc}",
                        oninput: move |e| fc.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Temp",
                        placeholder: "Temperatura",
                        value: "{temp}",
                        oninput: move |e| temp.set(e.value())
                    }
                    input {
                        class: "prompt",
                        name: "Spo2",
                        placeholder: "Saturação de Oxigênio",
                        value: "{spo2}",
                        oninput: move |e| spo2.set(e.value())
                    }
                }

                input {class:"button", r#type: "submit" }
            }

            div { class: "respostacont",
                h4 { class: "resposta_titulo", "Resultado" }

                match &*text.read(){
                    Some(Ok(result)) => {info!("{result}"); rsx!{p{class:"resposta", "{result}"}}},
                    Some(Err(e)) => {info!("{e}"); rsx!{p{class:"resposta", "{e}"}}},
                    None => rsx!{p{class:"resposta", "Sem Texto até o momento!"}}
                }
            }
        }
    }
}
