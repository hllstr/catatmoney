use dioxus::prelude::*;
use crate::components::icons::{
    CategoryIcon, IconArrowLeftRight, IconPaperclip, IconReceipt, IconTrash,
};
use crate::model::{format_idr, format_short_date, Transaction, TransactionType};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HistoryFilter {
    All,
    Expense,
    Income,
    Transfer,
}

#[component]
pub fn TransactionHistory(
    transactions: Vec<Transaction>,
    on_delete: EventHandler<Transaction>,
    on_select_trx: EventHandler<Transaction>,
    #[props(default = false)] is_private: bool,
) -> Element {
    let mut current_filter = use_signal(|| HistoryFilter::All);
    let mut search_query = use_signal(String::new);

    let filtered_list: Vec<Transaction> = transactions
        .into_iter()
        .filter(|t| {
            let matches_filter = match *current_filter.read() {
                HistoryFilter::All => true,
                HistoryFilter::Expense => t.transaction_type == TransactionType::Expense,
                HistoryFilter::Income => t.transaction_type == TransactionType::Income,
                HistoryFilter::Transfer => t.transaction_type == TransactionType::Transfer,
            };
            let q = search_query.read().to_lowercase();
            let matches_search = q.is_empty()
                || t.title.to_lowercase().contains(&q)
                || t.category.to_lowercase().contains(&q)
                || t.wallet.to_lowercase().contains(&q)
                || t.to_wallet.as_ref().map(|tw| tw.to_lowercase().contains(&q)).unwrap_or(false)
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
                        class: if *current_filter.read() == HistoryFilter::Expense { "filter-pill active" } else { "filter-pill" },
                        onclick: move |_| current_filter.set(HistoryFilter::Expense),
                        "Pengeluaran"
                    }
                    button {
                        r#type: "button",
                        class: if *current_filter.read() == HistoryFilter::Income { "filter-pill active" } else { "filter-pill" },
                        onclick: move |_| current_filter.set(HistoryFilter::Income),
                        "Pemasukan"
                    }
                    button {
                        r#type: "button",
                        class: if *current_filter.read() == HistoryFilter::Transfer { "filter-pill active" } else { "filter-pill" },
                        onclick: move |_| current_filter.set(HistoryFilter::Transfer),
                        "Transfer"
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
                            let (amount_str, amount_class, box_class) = match trx.transaction_type {
                                TransactionType::Income => (
                                    if is_private { "+Rp ••••••".to_string() } else { format!("+{}", format_idr(trx.amount)) },
                                    "row-amount tabular-numbers positive",
                                    "category-icon-box income",
                                ),
                                TransactionType::Expense => (
                                    if is_private { "-Rp ••••••".to_string() } else { format!("-{}", format_idr(trx.amount)) },
                                    "row-amount tabular-numbers negative",
                                    "category-icon-box expense",
                                ),
                                TransactionType::Transfer => (
                                    if is_private { "Rp ••••••".to_string() } else { format_idr(trx.amount) },
                                    "row-amount tabular-numbers transfer",
                                    "category-icon-box transfer",
                                ),
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
                                            if trx.transaction_type == TransactionType::Transfer {
                                                IconArrowLeftRight { size: "16" }
                                            } else {
                                                CategoryIcon { category: trx.category.clone() }
                                            }
                                        }
                                        div { class: "row-details",
                                            span { class: "row-title", "{trx.title}" }
                                            div { class: "row-meta",
                                                if trx.transaction_type == TransactionType::Transfer {
                                                    if let Some(ref dest) = trx.to_wallet {
                                                        span { class: "transfer-route-badge", "{trx.wallet} → {dest}" }
                                                    } else {
                                                        span { class: "meta-category", "{trx.category}" }
                                                    }
                                                } else {
                                                    span { class: "meta-category", "{trx.category}" }
                                                    span { "•" }
                                                    span { class: "meta-wallet-badge", "{trx.wallet}" }
                                                }
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
                                        if trx.transaction_type == TransactionType::Transfer {
                                            span { class: "flex items-center gap-1 {amount_class}",
                                                IconArrowLeftRight { size: "12" }
                                                "{amount_str}"
                                            }
                                        } else {
                                            span { class: "{amount_class}",
                                                "{amount_str}"
                                            }
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
