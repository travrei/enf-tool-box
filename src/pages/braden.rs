use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Braden() -> Element {
    let mut total = use_signal(|| 0);
    let mut show_percep = use_signal(|| false);
    let mut show_umid = use_signal(|| false);
    let mut show_ativ = use_signal(|| false);
    let mut show_mob = use_signal(|| false);
    let mut show_nut = use_signal(|| false);
    let mut show_fric = use_signal(|| false);
    let risco: String;

    match total() {
        0 => risco = "Sem Resultado".to_string(),
        1_i32..=9_i32 => risco = "Muito Alto".to_string(),
        10_i32..=12_i32 => risco = "Alto".to_string(),
        13_i32..=14_i32 => risco = "Moderado".to_string(),
        15_i32..=18_i32 => risco = "Baixo".to_string(),
        19_i32..=23_i32 => risco = "Sem Risco".to_string(),
        24_i32.. => risco = "Erro".to_string(),
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
                h1 { class: "navtitle", "Braden" }
                span {
                    button { class: "flatbutton", onclick: move |_| total.set(0),
                        "Zerar"
                        span { class: "icons material-icons", "highlight_off" }
                    }
                }
            }
            div { class: "items",
                if show_percep() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_percep.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_percep.set(false);
                            },
                            "Totalmente limitado - 1pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_percep.set(false);
                            },
                            "Muito limitado - 2pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_percep.set(false);
                            },
                            "Levemente limitado - 3pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_percep.set(false);
                            },
                            "Nenhuma limitação - 4pts"
                        }
                    }
                }

                button { class: "button", onclick: move |_| show_umid.set(true),
                    span { class: "icons material-icons", "water_drop" }
                    "Umidade"
                }
                if show_umid() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_umid.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_umid.set(false);
                            },
                            "Completamente Molhada - 1pt."
                        }

                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_umid.set(false);
                            },
                            "Muito molhada - 2pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_umid.set(false);
                            },
                            "Ocasionalmente molhada - 3pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_umid.set(false);
                            },
                            "Raramente molhada - 4pts."
                        }
                    }
                }

                button { class: "button", onclick: move |_| show_ativ.set(true),
                    span { class: "icons material-icons", "directions_walk" }
                    "Atividade"
                }
                if show_ativ() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_ativ.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_ativ.set(false);
                            },
                            "Acamado - 1pt."
                        }

                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_ativ.set(false);
                            },
                            "Confinado a Cadeira - 2pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_ativ.set(false);
                            },
                            "Anda ocasionalmente - 3pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_ativ.set(false);
                            },
                            "Anda frequentemente - 4pts."
                        }
                    }
                }
                button { class: "button", onclick: move |_| show_mob.set(true),
                    span { class: "icons material-icons", "wheelchair_pickup" }
                    "Mobilidade"
                }
                if show_mob() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_mob.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_mob.set(false);
                            },
                            "Totalmente imóvel - 1pt."
                        }

                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_mob.set(false);
                            },
                            "Bastante limitado - 2pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_mob.set(false);
                            },
                            "Levemente limitado - 3pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_mob.set(false);
                            },
                            "Não Apresenta limitações - 4pts."
                        }
                    }
                }

                button { class: "button", onclick: move |_| show_nut.set(true),
                    span { class: "icons material-icons", "restaurant_menu" }
                    "Nutrição"
                }
                if show_nut() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_nut.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_nut.set(false);
                            },
                            "Muito pobre - 1pt."
                        }

                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_nut.set(false);
                            },
                            "Provavelmente inadequado - 2pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_nut.set(false);
                            },
                            "Adequado - 3pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_nut.set(false);
                            },
                            "Excelente - 4pts."
                        }
                    }
                }

                button { class: "button", onclick: move |_| show_fric.set(true),
                    span { class: "icons material-icons", "compare_arrows" }
                    "Fricção e Cisalhamento"
                }
                if show_fric() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_fric.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_fric.set(false);
                            },
                            "Problema - 1pt."
                        }

                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_fric.set(false);
                            },
                            "Problema em potencial - 2pts."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_fric.set(false);
                            },
                            "Nenhum Problema - 3pts."
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
