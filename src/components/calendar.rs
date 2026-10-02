use dioxus::prelude::*;
use crate::components::icons::{
    CategoryIcon, IconCalendar, IconChevronLeft, IconChevronRight, IconPaperclip, IconTrash,
};
use crate::model::{format_idr, get_today_date, Transaction, TransactionType};

#[component]
pub fn FinancialCalendar(
    transactions: Vec<Transaction>,
    on_delete: EventHandler<Transaction>,
    on_select_trx: EventHandler<Transaction>,
) -> Element {
    let mut selected_date = use_signal(get_today_date);
    let mut current_year = use_signal(|| 2026);
    let mut current_month = use_signal(|| 10); // Oktober

    let month_names = [
        "Januari", "Februari", "Maret", "April", "Mei", "Juni",
        "Juli", "Agustus", "September", "Oktober", "November", "Desember",
    ];

    let month_label = format!("{} {}", month_names[*current_month.read() - 1], *current_year.read());

    // Hari dalam bulan (Oktober = 31 hari)
    let days_in_month = match *current_month.read() {
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    // Cari transaksi untuk tanggal yang dipilih
    let selected_trxs: Vec<Transaction> = transactions
        .iter()
        .filter(|t| t.date == *selected_date.read())
        .cloned()
        .collect();

    let (day_income, day_expense): (f64, f64) = selected_trxs.iter().fold((0.0, 0.0), |acc, t| {
        match t.transaction_type {
            TransactionType::Income => (acc.0 + t.amount, acc.1),
            TransactionType::Expense => (acc.0, acc.1 + t.amount),
        }
    });

    rsx! {
        div { class: "calendar-panel",
            div { class: "calendar-header",
                div { class: "flex items-center gap-2",
                    IconCalendar { size: "18" }
                    h2 { class: "panel-title", style: "margin: 0;", "Kalender Arus Kas" }
                }

                // Navigasi Bulan
                div { class: "month-navigator",
                    button {
                        r#type: "button",
                        class: "nav-arrow-btn",
                        onclick: move |_| {
                            let m = *current_month.read();
                            if m == 1 {
                                current_month.set(12);
                                let y = *current_year.read();
                                current_year.set(y - 1);
                            } else {
                                current_month.set(m - 1);
                            }
                        },
                        IconChevronLeft { size: "14" }
                    }
                    span { class: "month-display-text", "{month_label}" }
                    button {
                        r#type: "button",
                        class: "nav-arrow-btn",
                        onclick: move |_| {
                            let m = *current_month.read();
                            if m == 12 {
                                current_month.set(1);
                                let y = *current_year.read();
                                current_year.set(y + 1);
                            } else {
                                current_month.set(m + 1);
                            }
                        },
                        IconChevronRight { size: "14" }
                    }
                }
            }

            // Grid Nama Hari
            div { class: "calendar-weekdays",
                div { "Min" }
                div { "Sen" }
                div { "Sel" }
                div { "Rab" }
                div { "Kam" }
                div { "Jum" }
                div { "Sab" }
            }

            // Grid Tanggal
            div { class: "calendar-days-grid",
                for day in 1..=days_in_month {
                    {
                        let date_str = format!("{:04}-{:02}-{:02}", *current_year.read(), *current_month.read(), day);
                        let is_selected = *selected_date.read() == date_str;

                        // Cek apakah tanggal ini memiliki transaksi
                        let day_items: Vec<&Transaction> = transactions.iter().filter(|t| t.date == date_str).collect();
                        let has_income = day_items.iter().any(|t| t.transaction_type == TransactionType::Income);
                        let has_expense = day_items.iter().any(|t| t.transaction_type == TransactionType::Expense);

                        let mut cell_class = "calendar-day-cell".to_string();
                        if is_selected {
                            cell_class.push_str(" selected");
                        }

                        rsx! {
                            button {
                                key: "{date_str}",
                                r#type: "button",
                                class: "{cell_class}",
                                onclick: {
                                    let d = date_str.clone();
                                    move |_| selected_date.set(d.clone())
                                },
                                span { class: "day-number", "{day}" }

                                // Indikator Titik Arus Kas
                                if has_income || has_expense {
                                    div { class: "day-indicators",
                                        if has_income {
                                            span { class: "indicator-dot income" }
                                        }
                                        if has_expense {
                                            span { class: "indicator-dot expense" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Detail Transaksi Tanggal Terpilih
            div { class: "selected-date-detail",
                div { class: "selected-date-header",
                    div {
                        h4 { class: "selected-date-title", "Catatan: {*selected_date.read()}" }
                        p { class: "selected-date-meta tabular-numbers",
                            "Masuk: +{format_idr(day_income)} • Keluar: -{format_idr(day_expense)}"
                        }
                    }
                }

                if selected_trxs.is_empty() {
                    div { class: "feed-empty", style: "padding: 1.5rem;",
                        p { class: "feed-empty-text", "Tidak ada transaksi pada tanggal ini." }
                    }
                } else {
                    div { class: "transaction-feed",
                        for trx in selected_trxs {
                            {
                                let is_income = trx.transaction_type == TransactionType::Income;
                                let amount_str = if is_income {
                                    format!("+{}", format_idr(trx.amount))
                                } else {
                                    format!("-{}", format_idr(trx.amount))
                                };
                                let amount_class = if is_income {
                                    "row-amount tabular-numbers positive"
                                } else {
                                    "row-amount tabular-numbers negative"
                                };
                                let box_class = if is_income {
                                    "category-icon-box income"
                                } else {
                                    "category-icon-box expense"
                                };

                                rsx! {
                                    div {
                                        class: "transaction-row",
                                        key: "{trx.id}",
                                        onclick: {
                                            let t = trx.clone();
                                            move |_| on_select_trx.call(t.clone())
                                        },
                                        div { class: "row-left",
                                            div { class: "{box_class}",
                                                CategoryIcon { category: trx.category.clone() }
                                            }
                                            div { class: "row-details",
                                                span { class: "row-title", "{trx.title}" }
                                                div { class: "row-meta",
                                                    span { class: "meta-category", "{trx.category}" }
                                                    span { "•" }
                                                    span { class: "tabular-numbers", "{trx.time}" }
                                                    if let Some(ref att) = trx.attachment_name {
                                                        span { "•" }
                                                        span { class: "flex items-center gap-1 text-xs text-muted",
                                                            IconPaperclip { size: "12" }
                                                            "{att}"
                                                        }
                                                    }
                                                }
                                                if !trx.notes.is_empty() {
                                                    p { class: "row-notes", "{trx.notes}" }
                                                }
                                            }
                                        }

                                        div { class: "row-right",
                                            span { class: "{amount_class}", "{amount_str}" }
                                            button {
                                                r#type: "button",
                                                class: "btn-icon-danger",
                                                title: "Hapus Transaksi",
                                                onclick: {
                                                    let t = trx.clone();
                                                    move |e: MouseEvent| {
                                                        e.stop_propagation();
                                                        on_delete.call(t.clone());
                                                    }
                                                },
                                                IconTrash { size: "14" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
