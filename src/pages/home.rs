use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    let mut show_referencias = use_signal(|| false);

    rsx! {
        div { class: "container",
            div {
                h1 { class: "title", "NURSE KIT" }
                h4 { class: "secondary_title", " Tudo que você precisa para cuidar com excelência!" }
            }
            div { class: "items",
                Link { to: Route::Braden {}, class: "button", "Braden" }
                Link { to: Route::Morse {}, class: "button", "Morse" }
                Link { to: Route::Fugulin {}, class: "button", "Fugulin" }
                Link { to: Route::Glasgow {}, class: "button", "Glasgow" }
                Link { to: Route::Home {}, class: "button disable", "Gerador de Evolução (Em Breve)" }
            }
            div {
                button {
                    class: "button_sec",
                    onclick: move |_| show_referencias.set(true),
                    "Referências"
                }
            }
            if show_referencias() {
                button {
                    class: "opcoes_fundo",
                    onclick: move |_| show_referencias.set(false)
                }
                div { class: "opcoes_window",
                    div {
                        h3 { "Escala de Braden:" }
                        p {
                            "HEALTH SENSE Al. Braden Scale. Disponível em: "
                            a { href: "https://www.bradenscale.com", "https://www.bradenscale.com" }
                        }
                        h3 { "Escala de Morse:" }
                        p {
                            "MARTINEZ, M.C.; DALRI, M.C.B.; CHIANCA, T.C.M. Morse Fall Scale: tradução e adaptação. Revista da Escola de Enfermagem da USP, v.45, n.5, p.1200-1206, 2011"
                        }
                        h3 { "Escala de Fugulin:" }
                        p {
                            "COELHO, M.J.; PADILHA, M. I. C. S.; MOTTA, M. G. C. A humanização na assistência a saúde. Revista Latino-Americana de Enfermagem, v.13, n.1, p.117-121, 2005."
                        }
                        h3 { "Escala de Glasgow:" }
                        p {
                            "UNIVERSIDADE FEDERAL DE JUIZ DE FORA. Escala de coma de Glasgow: importância e atualização de 2018. Neurologia UFJF. 11 dez. 2018 Disponível em: "
                            a { href: "https://www2.ufjf.br/neurologia/2018/12/11/escala-de-coma-de-glasgow-importancia-e-atualizacao-de-2018",
                                "https://www2.ufjf.br/neurologia/2018/12/11/escala-de-coma-de-glasgow-importancia-e-atualizacao-de-2018"
                            }
                        }
                    }
                }
            }
        }
    }
}
