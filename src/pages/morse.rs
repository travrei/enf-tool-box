use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Morse() -> Element {
    let mut total = use_signal(|| 0);
    let mut show_queda = use_signal(|| false);
    let mut show_secnd = use_signal(|| false);
    let mut show_aumob = use_signal(|| false);
    let mut show_ev = use_signal(|| false);
    let mut show_marcha = use_signal(|| false);
    let mut show_estmental = use_signal(|| false);
    let risco: String;

    match total() {
        0_i32..=24_i32 => risco = "Baixo".to_string(),
        25_i32..=44_i32 => risco = "Médio".to_string(),
        45_i32..=140_i32 => risco = "Alto".to_string(),
        _ => risco = "Erro".to_string(),
    }

    rsx! {
        div { class: "container",
            nav {
                span {
                    Link { class: "flatbutton", to: Route::Home {},
                        span { class: "icons material-icons", "arrow_back" }
                        "Voltar"
                    }
                }
                h1 { class: "navtitle", "Morse" }
                span {
                    button { class: "flatbutton", onclick: move |_| total.set(0),
                        "Zerar"
                        span { class: "icons material-icons", "highlight_off" }
                    }
                }
            }
            div { class: "items",
                button { class: "button", onclick: move |_| show_queda.set(true),
                    span { class: "icons material-icons", "touch_app" }
                    "Histórico de Quedas"
                }
                if show_queda() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_queda.set(false)
                    }
                    div { class: "opcoes_window",
                        h3 { class: "nav_explan", "Nos ultimos 3 meses" }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 0;
                                show_queda.set(false);
                            },
                            "Não - 0pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 25;
                                show_queda.set(false);
                            },
                            "Sim - 25pts"
                        }
                    }
                }

                button { class: "button", onclick: move |_| show_secnd.set(true),
                    span { class: "icons material-icons", "timer" }
                    "Diagnóstico Secundário"
                }
                if show_secnd() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_secnd.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 0;
                                show_secnd.set(false);
                            },
                            "Não - 0pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 15;
                                show_secnd.set(false);
                            },
                            "Sim - 15pts"
                        }
                    }
                }

                button { class: "button", onclick: move |_| show_aumob.set(true),
                    span { class: "icons material-icons", "directions_walk" }
                    "Auxilio na Mobilidade"
                }
                if show_aumob() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_aumob.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 0;
                                show_aumob.set(false);
                            },
                            "Nenhum/Acamado/R. Leito - 0pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 15;
                                show_aumob.set(false);
                            },
                            "Bengala/Muleta- 15pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 30;
                                show_aumob.set(false);
                            },
                            "Apoia em Mobiliário ou Parede - 30pt."
                        }
                    }
                }

                button { class: "button", onclick: move |_| show_ev.set(true),
                    span { class: "icons material-icons", "medication" }
                    "Terapia Endovenosa"
                }
                if show_ev() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_ev.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 0;
                                show_ev.set(false);
                            },
                            "Não - 0pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 20;
                                show_ev.set(false);
                            },
                            "Sim - 20pts"
                        }
                    }
                }
                button { class: "button", onclick: move |_| show_marcha.set(true),
                    span { class: "icons material-icons", "directions_run" }
                    "Marcha"
                }
                if show_marcha() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_marcha.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 0;
                                show_marcha.set(false);
                            },
                            "Normal/Acamado/C.Rodas - 0pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 10;
                                show_marcha.set(false);
                            },
                            "Lenta/Fraca - 10pts"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 30;
                                show_marcha.set(false);
                            },
                            "Apoia em Mobiliário/Parede - 30pts"
                        }
                    }
                }

                button {
                    class: "button",
                    onclick: move |_| show_estmental.set(true),
                    span { class: "icons material-icons", "psychology" }
                    "Estado Mental"
                }
                if show_estmental() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_estmental.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 0;
                                show_estmental.set(false);
                            },
                            "Orientado - 0pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 20;
                                show_estmental.set(false);
                            },
                            "Desorientado/Confuso - 15pts"
                        }
                    }
                }
            }
            footer {
                p { class: "fotcontent", "Total: {total}" }
                p { class: "fotcontent", "Risco: {risco}" }
            }
        }
    }
}
