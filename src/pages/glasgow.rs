use dioxus::prelude::*;

use crate::Route;

pub fn Glasgow() -> Element {
    let mut show_abertura_ocular = use_signal(|| false);
    let mut show_resposta_verbal = use_signal(|| false);
    let mut show_resposta_motora = use_signal(|| false);
    let mut show_reatividade_pupilar = use_signal(|| false);

    let risco: String;
    let mut total = use_signal(|| 0);

    match total() {
        0 => risco = "Sem Resultado".to_string(),
        1_i32..=8_i32 => risco = "Grave".to_string(),
        9_i32..=12_i32 => risco = "Moderado".to_string(),
        13_i32..=15_i32 => risco = "Leve/Normal".to_string(),
        16_i32.. => risco = "Erro".to_string(),
        _ => risco = "Erro".to_string(),
    };

    rsx! {
        div { class: "container",
            nav {
                span {
                    Link { class: "flatbutton", to: Route::Home {},
                        span { class: "icons material-icons", "arrow_back" }
                        "Voltar"
                    }
                }
                h1 { class: "navtitle", "Glasgow" }
                span {
                    button { class: "flatbutton", onclick: move |_| total.set(0),
                        "Zerar"
                        span { class: "icons material-icons", "highlight_off" }
                    }
                }
            }
            div { class: "items",
                button {
                    class: "button",
                    onclick: move |_| show_abertura_ocular.set(true),
                    span { class: "icons material-icons", "visibility" }
                    "Abertura Ocular"
                }
                if show_abertura_ocular() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_abertura_ocular.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_abertura_ocular.set(false);
                            },
                            "Espontânea - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_abertura_ocular.set(false);
                            },
                            "Ao estímulo sonoro - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_abertura_ocular.set(false);
                            },
                            "Ao estímulo de pressão - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_abertura_ocular.set(false);
                            },
                            "Nenhuma - 1pt."
                        }
                    }
                }

                button {
                    class: "button",
                    onclick: move |_| show_resposta_verbal.set(true),
                    span { class: "icons material-icons", "record_voice_over" }
                    "Resposta Verbal"
                }
                if show_resposta_verbal() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_resposta_verbal.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 5;
                                show_resposta_verbal.set(false);
                            },
                            "Orientada - 5pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_resposta_verbal.set(false);
                            },
                            "Confusa - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_resposta_verbal.set(false);
                            },
                            "Verbaliza Palavras Soltas - 3pt"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_resposta_verbal.set(false);
                            },
                            "Verbaliza Sons - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_resposta_verbal.set(false);
                            },
                            "Nenhuma - 1pt."
                        }
                    }
                }

                button {
                    class: "button",
                    onclick: move |_| show_resposta_motora.set(true),
                    span { class: "icons material-icons", "directions_run" }
                    "Resposta Motora"
                }
                if show_resposta_motora() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_resposta_motora.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 6;
                                show_resposta_motora.set(false);
                            },
                            "Obedece a Comandos - 6pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 5;
                                show_resposta_motora.set(false);
                            },
                            "Localiza estímulo - 5pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_resposta_motora.set(false);
                            },
                            "Flexão normal - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_resposta_motora.set(false);
                            },
                            "Flexão anormal - 3pt"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_resposta_motora.set(false);
                            },
                            "Extensão anormal - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_resposta_motora.set(false);
                            },
                            "Nenhuma - 1pt."
                        }
                    }
                }

                button {
                    class: "button",
                    onclick: move |_| show_reatividade_pupilar.set(true),
                    span { class: "icons material-icons", "remove_red_eye" }
                    "Reatividade Pupilar"
                }
                if show_reatividade_pupilar() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_reatividade_pupilar.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total -= 2;
                                show_reatividade_pupilar.set(false);
                            },
                            "Inexistente - (-2pt.)"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total -= 1;
                                show_reatividade_pupilar.set(false);
                            },
                            "Unilateral - (-1pt.)"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 0;
                                show_reatividade_pupilar.set(false);
                            },
                            "Bilateral - 0pt."
                        }
                    }
                }
            }
            footer {
                p { class: "fotcontent", "Total: {total}" }
                p { class: "fotcontent", "Trauma: {risco}" }
            }
        }
    }
}
