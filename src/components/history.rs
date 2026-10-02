use dioxus::prelude::*;
use crate::components::icons::{CategoryIcon, IconPaperclip, IconReceipt, IconTrash};
use crate::model::{format_idr, format_short_date, Transaction, TransactionType};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HistoryFilter {
    All,
    Income,
    Expense,
}

#[component]
pub fn TransactionHistory(
    transactions: Vec<Transaction>,
    on_delete: EventHandler<Transaction>,
    on_select_trx: EventHandler<Transaction>,
) -> Element {
    let mut current_filter = use_signal(|| HistoryFilter::All);
    let mut search_query = use_signal(String::new);

    let filtered_list: Vec<Transaction> = transactions
        .into_iter()
        .filter(|t| {
            let matches_filter = match *current_filter.read() {
                HistoryFilter::All => true,
                HistoryFilter::Income => t.transaction_type == TransactionType::Income,
                HistoryFilter::Expense => t.transaction_type == TransactionType::Expense,
            };
            let q = search_query.read().to_lowercase();
            let matches_search = q.is_empty()
                || t.title.to_lowercase().contains(&q)
                || t.category.to_lowercase().contains(&q)
                || t.wallet.to_lowercase().contains(&q)
                || t.notes.to_lowercase().contains(&q);

            matches_filter && matches_search
        })
        .collect();

    rsx! {
        div { class: "surface-panel",
            div { class: "panel-header flex flex-wrap gap-3 justify-between items-center",
                h2 { class: "panel-title",
                    IconReceipt { size: "16" }
                    "Riwayat Transaksi Lengkap"
                }

                // Filter Pills
                div { class: "filter-pills",
                    button {
                        r#type: "button",
                        class: if *current_filter.read() == HistoryFilter::All { "filter-pill active" } else { "filter-pill" },
                        onclick: move |_| current_filter.set(HistoryFilter::All),
                        "Semua"
                    }
                    button {
                        r#type: "button",
                        class: if *current_filter.read() == HistoryFilter::Income { "filter-pill active" } else { "filter-pill" },
                        onclick: move |_| current_filter.set(HistoryFilter::Income),
                        "Pemasukan"
                    }
                    button {
                        r#type: "button",
                        class: if *current_filter.read() == HistoryFilter::Expense { "filter-pill active" } else { "filter-pill" },
                        onclick: move |_| current_filter.set(HistoryFilter::Expense),
                        "Pengeluaran"
                    }
                }
            }

            // Search Bar Minimalis
            div { class: "mb-3",
                input {
                    class: "field-input text-xs",
                    r#type: "text",
                    placeholder: "Cari transaksi berdasarkan judul, kategori, atau catatan...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            if filtered_list.is_empty() {
                div { class: "feed-empty",
                    div { class: "feed-empty-icon",
                        IconReceipt { size: "22" }
                    }
                    p { class: "feed-empty-text", "Tidak ada transaksi yang cocok dengan kriteria pencarian." }
                }
            } else {
                div { class: "transaction-feed",
                    for trx in filtered_list {
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
                                                span { class: "meta-wallet-badge", "{trx.wallet}" }
                                                span { class: "inline sm:hidden tabular-numbers text-muted", "• {format_short_date(&trx.date)}" }
                                                span { class: "hidden sm:inline tabular-numbers text-muted", "• {trx.date} • {trx.time}" }
                                                if trx.attachment_name.is_some() {
                                                    span { class: "inline-flex items-center text-muted ml-0.5",
                                                        IconPaperclip { size: "11" }
                                                    }
                                                }
                                            }
                                            if !trx.notes.is_empty() {
                                                p { class: "row-notes", "{trx.notes}" }
                                            }
                                        }
                                    }

                                    div { class: "row-right",
                                        span { class: "{amount_class}",
                                            "{amount_str}"
                                        }
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
