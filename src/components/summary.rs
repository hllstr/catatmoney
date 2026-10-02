use dioxus::prelude::*;
use crate::components::icons::{IconArrowDownRight, IconArrowUpRight, IconWallet};
use crate::model::format_idr;

#[component]
pub fn Summary(balance: f64, income: f64, expense: f64) -> Element {
    let balance_class = if balance >= 0.0 {
        "metric-value tabular-numbers"
    } else {
        "metric-value tabular-numbers negative"
    };

    let savings_rate = if income > 0.0 {
        ((income - expense) / income * 100.0).clamp(-100.0, 100.0)
    } else {
        0.0
    };

    rsx! {
        div { class: "summary-container",
            // Kartu Total Saldo Bersih (Hero Balance)
            div { class: "metric-card hero-balance",
                div { class: "metric-card-top",
                    span { class: "metric-title", "Total Saldo Bersih" }
                    div { class: "metric-icon-indicator",
                        IconWallet { size: "16" }
                    }
                }
                div { class: "{balance_class}",
                    "{format_idr(balance)}"
                }
                div { class: "metric-sub-badge tabular-numbers",
                    span { "Rasio Tabungan: {savings_rate:.1}%" }
                }
            }

            // Kartu Total Pemasukan (Cash Inflow)
            div { class: "metric-card",
                div { class: "metric-card-top",
                    span { class: "metric-title", "Total Pemasukan" }
                    div { class: "metric-icon-indicator positive",
                        IconArrowUpRight { size: "15" }
                    }
                }
                div { class: "metric-value tabular-numbers positive",
                    "+{format_idr(income)}"
                }
                div { class: "text-xs text-muted mt-1",
                    "Arus Kas Masuk Terverifikasi"
                }
            }

            // Kartu Total Pengeluaran (Cash Outflow)
            div { class: "metric-card",
                div { class: "metric-card-top",
                    span { class: "metric-title", "Total Pengeluaran" }
                    div { class: "metric-icon-indicator negative",
                        IconArrowDownRight { size: "15" }
                    }
                }
                div { class: "metric-value tabular-numbers negative",
                    "-{format_idr(expense)}"
                }
                div { class: "text-xs text-muted mt-1",
                    "Total Belanja & Biaya Hidup"
                }
            }
        }
    }
}
