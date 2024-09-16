use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Fugulin() -> Element {
    let mut total = use_signal(|| 0);
    let mut show_estmental = use_signal(|| false);
    let mut show_oxigenacao = use_signal(|| false);
    let mut show_sinaisvitais = use_signal(|| false);
    let mut show_motilidade = use_signal(|| false);
    let mut show_deambulacao = use_signal(|| false);
    let mut show_alimentacao = use_signal(|| false);
    let mut show_cuidadocorporal = use_signal(|| false);
    let mut show_eliminação = use_signal(|| false);
    let mut show_terapeutica = use_signal(|| false);
    let mut show_comprometimentotecidual = use_signal(|| false);
    let mut show_curativo = use_signal(|| false);
    let mut show_tempodecurativo = use_signal(|| false);

    let risco: String;

    match total() {
        0_i32..=8_i32 => risco = "Sem Resultado".to_string(),
        9_i32..=14_i32 => risco = "Mínimo".to_string(),
        15_i32..=20_i32 => risco = "Intermediário".to_string(),
        21_i32..=26_i32 => risco = "Alta dependência".to_string(),
        27_i32..=31_i32 => risco = "Semi-intensivo".to_string(),
        32_i32..=48_i32 => risco = "Intensivo".to_string(),
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
                h1 { class: "navtitle", "Fugulin" }
                span {
                    button { class: "flatbutton", onclick: move |_| total.set(0),
                        "Zerar"
                        span { class: "icons material-icons", "highlight_off" }
                    }
                }
            }
            div { class: "items",
                // Estado Mental
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
                                total += 4;
                                show_estmental.set(false);
                            },
                            "Inconsciente - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_estmental.set(false);
                            },
                            "Períodos de Inconsciencia - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_estmental.set(false);
                            },
                            "Periodos de Desorientação - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_estmental.set(false);
                            },
                            "Orientado - 1pt."
                        }
                    }
                }
                // Oxigenação
                button {
                    class: "button",
                    onclick: move |_| show_oxigenacao.set(true),
                    span { class: "icons material-symbols-outlined", "spo2" }
                    "Oxigenação"
                }
                if show_oxigenacao() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_oxigenacao.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_oxigenacao.set(false);
                            },
                            "Ventilação Mecânica - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_oxigenacao.set(false);
                            },
                            "Mascara contínua ou Cateter de Oxigênio - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_oxigenacao.set(false);
                            },
                            "Mascara intermitente ou Cateter de Oxigênio - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_oxigenacao.set(false);
                            },
                            "Não depende de oxigênio - 1pt."
                        }
                    }
                }

                // Sinais Vitais
                button {
                    class: "button",
                    onclick: move |_| show_sinaisvitais.set(true),
                    span { class: "icons material-symbols-outlined", "vital_signs" }
                    "Sinais Vitais"
                }
                if show_sinaisvitais() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_sinaisvitais.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_sinaisvitais.set(false);
                            },
                            "Controles ≤ 2 horas- 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_sinaisvitais.set(false);
                            },
                            "Controle de 4/4 horas - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_sinaisvitais.set(false);
                            },
                            "Controle de 6/6 horas - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_sinaisvitais.set(false);
                            },
                            "Controle de 8/8 horas - 1pt."
                        }
                    }
                }

                // Motilidade
                button {
                    class: "button",
                    onclick: move |_| show_motilidade.set(true),
                    span { class: "icons material-icons", "directions_run" }
                    "Motilidade"
                }
                if show_motilidade() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_motilidade.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_motilidade.set(false);
                            },
                            "Incapaz de movimentar qualquer parte do corpo - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_motilidade.set(false);
                            },
                            "Dificuldade de movimentar segmentos corporais - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_motilidade.set(false);
                            },
                            "Limitação de movimentos - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_motilidade.set(false);
                            },
                            "Movimenta todas as partes do corpo - 1pt."
                        }
                    }
                }

                // Deambulação
                button {
                    class: "button",
                    onclick: move |_| show_deambulacao.set(true),
                    span { class: "icons material-icons", "directions_walk" }
                    "Deambulação"
                }
                if show_deambulacao() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_deambulacao.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_deambulacao.set(false);
                            },
                            "Restrito ao leito - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_deambulacao.set(false);
                            },
                            "Cadeira de rodas - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_deambulacao.set(false);
                            },
                            "Auxílio para deambular - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_deambulacao.set(false);
                            },
                            "Ambulante - 1pt."
                        }
                    }
                }

                // Alimentação
                button {
                    class: "button",
                    onclick: move |_| show_alimentacao.set(true),
                    span { class: "icons material-icons", "restaurant" }
                    "Alimentação"
                }
                if show_alimentacao() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_alimentacao.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_alimentacao.set(false);
                            },
                            "Cateter Central - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_alimentacao.set(false);
                            },
                            "Sonda Nasogástrica - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_alimentacao.set(false);
                            },
                            "Via oral, com auxílio - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_alimentacao.set(false);
                            },
                            "Auto suficiente - 1pt."
                        }
                    }
                }

                // Cuidado Corporal
                button {
                    class: "button",
                    onclick: move |_| show_cuidadocorporal.set(true),
                    span { class: "icons material-icons", "shower" }
                    "Cuidado Corporal"
                }
                if show_cuidadocorporal() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_cuidadocorporal.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_cuidadocorporal.set(false);
                            },
                            "Banho no Leito, Higiene Oral pela Enf. - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_cuidadocorporal.set(false);
                            },
                            "Banho de asperção, Higiene oral pela Enf. - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_cuidadocorporal.set(false);
                            },
                            "Auxílio no banho ou higene oral - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_cuidadocorporal.set(false);
                            },
                            "Auto suficiente - 1pt."
                        }
                    }
                }

                // Eliminação
                button {
                    class: "button",
                    onclick: move |_| show_eliminação.set(true),
                    span { class: "icons material-icons", "wc" }
                    "Eliminação"
                }
                if show_eliminação() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_eliminação.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_eliminação.set(false);
                            },
                            "Evacuação no leito, Uso de sonda para diurese- 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_eliminação.set(false);
                            },
                            "Uso de comadre Eliminações no leito - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_eliminação.set(false);
                            },
                            "Uso do vaso sanitário com auxílio - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_eliminação.set(false);
                            },
                            "Auto suficiente - 1pt."
                        }
                    }
                }

                // Terapêutica
                button {
                    class: "button",
                    onclick: move |_| show_terapeutica.set(true),
                    span { class: "icons material-icons", "medication" }
                    "Terapêutica"
                }
                if show_terapeutica() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_terapeutica.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_terapeutica.set(false);
                            },
                            "Uso de drogas vasoativas - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_terapeutica.set(false);
                            },
                            "E.V. contínuo ou sonda nasogástrica - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_terapeutica.set(false);
                            },
                            "E.V. Intermitente - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_terapeutica.set(false);
                            },
                            "I.M. ou V.O. - 1pt."
                        }
                    }
                }

                // Comprometimento Tecidual
                button {
                    class: "button",
                    onclick: move |_| show_comprometimentotecidual.set(true),
                    span { class: "icons material-symbols-outlined", "dermatology" }
                    "Comprometimento Tecidual"
                }
                if show_comprometimentotecidual() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_comprometimentotecidual.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_comprometimentotecidual.set(false);
                            },
                            "Destituição da derme, epiderme, musculos e demais estruturas - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_comprometimentotecidual.set(false);
                            },
                            "Tecido subcutâneo e músculo. Incisão cirúrgica, ostomias, drenos - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_comprometimentotecidual.set(false);
                            },
                            "Alteração da cor da pele e/ou epiderme, derme ou ambos - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_comprometimentotecidual.set(false);
                            },
                            "Pele íntegra - 1pt."
                        }
                    }
                }

                // Curativo
                button {
                    class: "button",
                    onclick: move |_| show_curativo.set(true),
                    span { class: "icons material-symbols-outlined", "healing" }
                    "Curativo"
                }
                if show_curativo() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_curativo.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_curativo.set(false);
                            },
                            "3 vezes ou mais ao dia - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_curativo.set(false);
                            },
                            "2 vezes ao dia - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_curativo.set(false);
                            },
                            "1 vez ao dia - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_curativo.set(false);
                            },
                            "Sem curativo - 1pt."
                        }
                    }
                }

                // Tempo de Curativo
                button {
                    class: "button",
                    onclick: move |_| show_tempodecurativo.set(true),
                    span { class: "icons material-icons", "timer" }
                    "Tempo de Curativo"
                }
                if show_tempodecurativo() {
                    button {
                        class: "opcoes_fundo",
                        onclick: move |_| show_tempodecurativo.set(false)
                    }
                    div { class: "opcoes_window",
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 4;
                                show_tempodecurativo.set(false);
                            },
                            "Superior a 30minutos - 4pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 3;
                                show_tempodecurativo.set(false);
                            },
                            "Entre 15 e 30 minutos - 3pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 2;
                                show_tempodecurativo.set(false);
                            },
                            "Entre 5 e 15 minutos - 2pt."
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                total += 1;
                                show_tempodecurativo.set(false);
                            },
                            "Sem curativo - 1pt."
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
