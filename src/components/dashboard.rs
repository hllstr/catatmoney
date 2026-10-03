use dioxus::prelude::*;
use crate::components::chart::CashflowChart;
use crate::components::icons::{
    CategoryIcon, IconArrowRight, IconPieChart, IconPlus, IconReceipt, IconWallet, WalletIcon,
};
use crate::components::summary::Summary;
use crate::model::{calculate_wallet_balance, format_idr, Transaction, TransactionType, Wallet};
use std::collections::HashMap;

#[component]
pub fn MainDashboard(
    user_name: String,
    transactions: Vec<Transaction>,
    wallets: Vec<Wallet>,
    on_go_to_history: EventHandler<()>,
    on_select_trx: EventHandler<Transaction>,
    on_open_add_wallet: EventHandler<()>,
) -> Element {
    // Hitung ringkasan
    let (total_income, total_expense) = transactions.iter().fold((0.0, 0.0), |acc, t| {
        match t.transaction_type {
            TransactionType::Income => (acc.0 + t.amount, acc.1),
            TransactionType::Expense => (acc.0, acc.1 + t.amount),
        }
    });
    // Total saldo bersih dihitung dari seluruh pos saldo sumber dana yang aktif
    let total_balance: f64 = wallets.iter().map(|w| calculate_wallet_balance(w, &transactions)).sum();

    // Breakdown Pengeluaran per Kategori
    let mut category_expense: HashMap<String, f64> = HashMap::new();
    for t in &transactions {
        if t.transaction_type == TransactionType::Expense {
            *category_expense.entry(t.category.clone()).or_insert(0.0) += t.amount;
        }
    }
    let mut sorted_expenses: Vec<(String, f64)> = category_expense.into_iter().collect();
    sorted_expenses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let recent_transactions: Vec<Transaction> = transactions.iter().take(4).cloned().collect();

    rsx! {
        div { class: "dashboard-container",
            // 0. Greeting Header Personal
            div { class: "greeting-banner mb-6 p-4 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-col sm:flex-row sm:items-center justify-between gap-3",
                div {
                    h2 { class: "text-lg font-bold text-[var(--text-primary)] tracking-tight", "Halo, {user_name}" }
                    p { class: "text-xs text-[var(--text-secondary)] mt-0.5", "Berikut ringkasan arus kas dan posisi saldo seluruh akun keuangan Anda." }
                }
            }

            // 1. Top Cards Ringkasan Keuangan
            Summary {
                balance: total_balance,
                income: total_income,
                expense: total_expense,
            }

            // 2. Sumber Dana & Akun Keuangan
            div { class: "surface-panel mb-6",
                div { class: "panel-header flex items-center justify-between",
                    h3 { class: "panel-title",
                        IconWallet { size: "16" }
                        "Sumber Dana & Dompet"
                    }
                    button {
                        r#type: "button",
                        class: "btn-secondary text-xs flex items-center gap-1.5 py-1.5 px-3 shrink-0 whitespace-nowrap",
                        onclick: move |_| on_open_add_wallet.call(()),
                        IconPlus { size: "13" }
                        span { "Tambah Akun" }
                    }
                }
                div { class: "wallet-grid",
                    for w in wallets.iter() {
                        {
                            let current_bal = calculate_wallet_balance(w, &transactions);
                            let bal_class = if current_bal >= 0.0 {
                                "wallet-balance tabular-numbers positive"
                            } else {
                                "wallet-balance tabular-numbers negative"
                            };

                            rsx! {
                                div { class: "wallet-card", key: "{w.id}",
                                    div { class: "wallet-card-header",
                                        div { class: "wallet-icon-box",
                                            WalletIcon { wallet_type: w.wallet_type, size: "15" }
                                        }
                                        span { class: "wallet-type-badge", "{w.wallet_type.as_str()}" }
                                    }
                                    div { class: "wallet-name", "{w.name}" }
                                    div { class: "{bal_class}", "{format_idr(current_bal)}" }
                                    div { class: "wallet-initial tabular-numbers",
                                        "Saldo Awal: {format_idr(w.initial_balance)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 3. Baris Utama: Grafik Tren Arus Kas & Breakdown Kategori
            div { class: "dashboard-main-grid",
                // Kolom Kiri: Grafik Vektor SVG
                div { class: "surface-panel",
                    CashflowChart { transactions: transactions.clone() }
                }

                // Kolom Kanan: Distribusi Kategori Pengeluaran
                div { class: "surface-panel",
                    div { class: "panel-header",
                        h3 { class: "panel-title",
                            IconPieChart { size: "16" }
                            "Alokasi Pengeluaran"
                        }
                    }

                    if sorted_expenses.is_empty() {
                        div { class: "feed-empty", style: "padding: 2rem 1rem;",
                            p { class: "feed-empty-text", "Belum ada data pengeluaran." }
                        }
                    } else {
                        div { class: "category-breakdown-list",
                            for (cat, amt) in sorted_expenses.iter().take(5) {
                                {
                                    let percentage = if total_expense > 0.0 {
                                        (*amt / total_expense * 100.0).clamp(0.0, 100.0)
                                    } else {
                                        0.0
                                    };

                                    rsx! {
                                        div { class: "breakdown-item", key: "{cat}",
                                            div { class: "breakdown-meta",
                                                span { class: "breakdown-name", "{cat}" }
                                                span { class: "breakdown-amount tabular-numbers",
                                                    "{format_idr(*amt)} ({percentage:.1}%)"
                                                }
                                            }
                                            div { class: "breakdown-bar-track",
                                                div {
                                                    class: "breakdown-bar-fill",
                                                    style: "width: {percentage}%;",
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

            // 3. Aktivitas Transaksi Terkini
            div { class: "surface-panel mt-6",
                div { class: "panel-header",
                    h3 { class: "panel-title",
                        IconReceipt { size: "16" }
                        "Aktivitas Transaksi Terbaru"
                    }
                    button {
                        r#type: "button",
                        class: "btn-link text-xs flex items-center gap-1",
                        onclick: move |_| on_go_to_history.call(()),
                        "Lihat Semua"
                        IconArrowRight { size: "12" }
                    }
                }

                if recent_transactions.is_empty() {
                    div { class: "feed-empty", style: "padding: 2.5rem 1rem;",
                        div { class: "feed-empty-icon mb-2 flex justify-center text-[var(--text-muted)]",
                            IconReceipt { size: "24" }
                        }
                        p { class: "feed-empty-text text-xs text-[var(--text-secondary)] max-w-sm mx-auto text-center leading-relaxed",
                            "Belum ada transaksi tercatat. Tekan tombol (+) di bilah navigasi bawah untuk mencatat transaksi pertama Anda."
                        }
                    }
                } else {
                    div { class: "transaction-feed",
                        for trx in recent_transactions {
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
                                                    span { "{trx.date}" }
                                                    span { "•" }
                                                    span { class: "tabular-numbers", "{trx.time}" }
                                                }
                                            }
                                        }

                                        div { class: "row-right",
                                            span { class: "{amount_class}", "{amount_str}" }
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
