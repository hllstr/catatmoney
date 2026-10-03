use dioxus::prelude::*;
use crate::components::chart::CashflowChart;
use crate::components::icons::{
    CategoryIcon, IconArrowLeftRight, IconArrowRight, IconPieChart, IconPiggyBank, IconPlus, IconReceipt,
    IconTarget, IconWallet, WalletIcon,
};
use crate::components::summary::Summary;
use crate::model::{
    calculate_wallet_balance, format_idr, get_month_days_info, get_today_date,
    CategoryBudget, SavingsGoal, Transaction, TransactionType, Wallet,
};
use std::collections::HashMap;

#[component]
pub fn MainDashboard(
    user_name: String,
    transactions: Vec<Transaction>,
    wallets: Vec<Wallet>,
    budgets: Vec<CategoryBudget>,
    savings_goals: Vec<SavingsGoal>,
    on_go_to_history: EventHandler<()>,
    on_select_trx: EventHandler<Transaction>,
    on_open_add_wallet: EventHandler<()>,
    on_go_to_analytics: EventHandler<()>,
    on_go_to_budget: EventHandler<()>,
    on_go_to_savings: EventHandler<()>,
) -> Element {
    // Hitung ringkasan
    let (total_income, total_expense) = transactions.iter().fold((0.0, 0.0), |acc, t| {
        match t.transaction_type {
            TransactionType::Income => (acc.0 + t.amount, acc.1),
            TransactionType::Expense => (acc.0, acc.1 + t.amount),
            TransactionType::Transfer => (acc.0, acc.1 + t.admin_fee.unwrap_or(0.0)),
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

    // Perhitungan Anggaran Bulan Berjalan
    let today = get_today_date();
    let current_month_prefix = if today.len() >= 7 { &today[0..7] } else { "2026-10" };
    let (current_day, total_days_in_month) = get_month_days_info();
    let remaining_days = total_days_in_month.saturating_sub(current_day) + 1;

    let mut month_category_spent: HashMap<String, f64> = HashMap::new();
    for t in &transactions {
        if t.transaction_type == TransactionType::Expense && t.date.starts_with(current_month_prefix) {
            *month_category_spent.entry(t.category.clone()).or_insert(0.0) += t.amount;
        }
    }

    let total_budget_limit: f64 = budgets.iter().map(|b| b.monthly_limit).sum();
    let total_budget_spent: f64 = budgets
        .iter()
        .map(|b| month_category_spent.get(&b.category).cloned().unwrap_or(0.0))
        .sum();
    let total_budget_remaining = total_budget_limit - total_budget_spent;
    let budget_usage_pct = if total_budget_limit > 0.0 {
        (total_budget_spent / total_budget_limit * 100.0).clamp(0.0, 999.0)
    } else {
        0.0
    };
    let daily_safe_spend = if total_budget_remaining > 0.0 {
        total_budget_remaining / remaining_days as f64
    } else {
        0.0
    };

    // Perhitungan Ringkasan Target Tabungan
    let dash_total_target: f64 = savings_goals.iter().map(|g| g.target_amount).sum();
    let dash_total_saved: f64 = savings_goals.iter().map(|g| g.current_amount).sum();
    let dash_savings_pct = if dash_total_target > 0.0 {
        (dash_total_saved / dash_total_target * 100.0).clamp(0.0, 999.0)
    } else {
        0.0
    };
    let active_savings_count = savings_goals.iter().filter(|g| g.current_amount < g.target_amount).count();
    let completed_savings_count = savings_goals.iter().filter(|g| g.current_amount >= g.target_amount).count();

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
                                        span { class: "wallet-type-text", "{w.wallet_type.as_str()}" }
                                    }
                                    div { class: "wallet-name", "{w.name}" }
                                    div { class: "{bal_class}", "{format_idr(current_bal)}" }
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
                    div { class: "panel-header flex items-center justify-between",
                        h3 { class: "panel-title",
                            IconPieChart { size: "16" }
                            "Alokasi Pengeluaran"
                        }
                        button {
                            r#type: "button",
                            class: "btn-link text-xs flex items-center gap-1",
                            onclick: move |_| on_go_to_analytics.call(()),
                            "Lihat Detail"
                            IconArrowRight { size: "12" }
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

            // 4. Panel Ringkasan Anggaran & Batas Belanja Bulanan
            if !budgets.is_empty() {
                div { class: "surface-panel mt-6 p-4 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)]",
                    div { class: "panel-header flex items-center justify-between mb-3",
                        div { class: "flex items-center gap-2",
                            span { class: "w-7 h-7 rounded-lg bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)]",
                                IconTarget { size: "15" }
                            }
                            div {
                                h3 { class: "text-sm font-bold text-[var(--text-primary)]", "Status Anggaran Bulan Ini" }
                                p { class: "text-[11px] text-[var(--text-muted)]",
                                    "Pagu harian aman: {format_idr(daily_safe_spend)}/hari ({remaining_days} hari tersisa)"
                                }
                            }
                        }
                        button {
                            r#type: "button",
                            class: "btn-link text-xs flex items-center gap-1",
                            onclick: move |_| on_go_to_budget.call(()),
                            "Kelola Anggaran"
                            IconArrowRight { size: "12" }
                        }
                    }

                    // Progress Bar Utama
                    div { class: "space-y-1.5 my-2",
                        div { class: "flex items-center justify-between text-xs",
                            span { class: "text-[var(--text-secondary)]",
                                "Terpakai: "
                                strong { class: "text-[var(--text-primary)] tabular-numbers", "{format_idr(total_budget_spent)}" }
                                " dari {format_idr(total_budget_limit)}"
                            }
                            span { class: "font-semibold tabular-numbers text-[var(--text-primary)]",
                                "{budget_usage_pct:.0}%"
                            }
                        }
                        div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-2 overflow-hidden",
                            div {
                                class: if budget_usage_pct >= 100.0 { "bg-[var(--negative)] h-full rounded-full transition-all" } else if budget_usage_pct >= 75.0 { "bg-[var(--amber-500)] h-full rounded-full transition-all" } else { "bg-[var(--positive)] h-full rounded-full transition-all" },
                                style: "width: {budget_usage_pct.min(100.0)}%;",
                            }
                        }
                    }

                    // Mini Progress per Kategori (maksimal 3 kategori teratas)
                    div { class: "grid grid-cols-1 sm:grid-cols-3 gap-2.5 pt-3 mt-2 border-t border-[var(--border-subtle)]",
                        for b in budgets.iter().take(3) {
                            {
                                let b_spent = month_category_spent.get(&b.category).cloned().unwrap_or(0.0);
                                let b_pct = if b.monthly_limit > 0.0 { (b_spent / b.monthly_limit * 100.0).clamp(0.0, 100.0) } else { 0.0 };
                                rsx! {
                                    div { class: "p-2 rounded-lg bg-[var(--bg-surface-subtle)] text-[11px]",
                                        key: "{b.id}",
                                        div { class: "flex items-center justify-between mb-1",
                                            span { class: "font-semibold text-[var(--text-primary)] truncate max-w-[120px]", "{b.category}" }
                                            span { class: "tabular-numbers text-[var(--text-muted)]", "{b_pct:.0}%" }
                                        }
                                        div { class: "w-full bg-[var(--bg-app)] rounded-full h-1 overflow-hidden",
                                            div {
                                                class: if b_pct >= 100.0 { "bg-[var(--negative)] h-full rounded-full" } else if b_pct >= 75.0 { "bg-[var(--amber-500)] h-full rounded-full" } else { "bg-[var(--positive)] h-full rounded-full" },
                                                style: "width: {b_pct}%;",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 5. Panel Ringkasan Target Tabungan
            if !savings_goals.is_empty() {
                div { class: "surface-panel mt-6 p-4 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)]",
                    div { class: "panel-header flex items-center justify-between mb-3",
                        div { class: "flex items-center gap-2",
                            span { class: "w-7 h-7 rounded-lg bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--accent)]",
                                IconPiggyBank { size: "15" }
                            }
                            div {
                                h3 { class: "text-sm font-bold text-[var(--text-primary)]", "Progres Target Tabungan" }
                                p { class: "text-[11px] text-[var(--text-muted)]",
                                    "{active_savings_count} sasaran aktif, {completed_savings_count} tercapai"
                                }
                            }
                        }
                        button {
                            r#type: "button",
                            class: "btn-link text-xs flex items-center gap-1",
                            onclick: move |_| on_go_to_savings.call(()),
                            "Buka Tabungan"
                            IconArrowRight { size: "12" }
                        }
                    }

                    // Progress Bar Utama Tabungan
                    div { class: "space-y-1.5 my-2",
                        div { class: "flex items-center justify-between text-xs",
                            span { class: "text-[var(--text-secondary)]",
                                "Terkumpul: "
                                strong { class: "text-[var(--text-primary)] tabular-numbers", "{format_idr(dash_total_saved)}" }
                                " dari {format_idr(dash_total_target)}"
                            }
                            span { class: "font-semibold tabular-numbers text-[var(--accent)]",
                                "{dash_savings_pct:.0}%"
                            }
                        }
                        div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-2 overflow-hidden",
                            div {
                                class: "bg-[var(--accent)] h-full rounded-full transition-all",
                                style: "width: {dash_savings_pct.min(100.0)}%;",
                            }
                        }
                    }

                    // Mini Progress per Target (maksimal 3 target teratas)
                    div { class: "grid grid-cols-1 sm:grid-cols-3 gap-2.5 pt-3 mt-2 border-t border-[var(--border-subtle)]",
                        for goal in savings_goals.iter().take(3) {
                            {
                                let g_pct = if goal.target_amount > 0.0 {
                                    (goal.current_amount / goal.target_amount * 100.0).clamp(0.0, 100.0)
                                } else {
                                    0.0
                                };
                                let is_done = goal.current_amount >= goal.target_amount;
                                rsx! {
                                    div { class: "p-2 rounded-lg bg-[var(--bg-surface-subtle)] text-[11px]",
                                        key: "{goal.id}",
                                        div { class: "flex items-center justify-between mb-1",
                                            span { class: "font-semibold text-[var(--text-primary)] truncate max-w-[120px]", "{goal.name}" }
                                            span { class: if is_done { "tabular-numbers text-[var(--positive)] font-bold" } else { "tabular-numbers text-[var(--text-muted)]" },
                                                "{g_pct:.0}%"
                                            }
                                        }
                                        div { class: "w-full bg-[var(--bg-app)] rounded-full h-1 overflow-hidden",
                                            div {
                                                class: if is_done { "bg-[var(--positive)] h-full rounded-full" } else { "bg-[var(--accent)] h-full rounded-full" },
                                                style: "width: {g_pct}%;",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 6. Aktivitas Transaksi Terkini
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
                                let (amount_str, amount_class, box_class) = match trx.transaction_type {
                                    TransactionType::Income => (
                                        format!("+{}", format_idr(trx.amount)),
                                        "row-amount tabular-numbers positive",
                                        "category-icon-box income",
                                    ),
                                    TransactionType::Expense => (
                                        format!("-{}", format_idr(trx.amount)),
                                        "row-amount tabular-numbers negative",
                                        "category-icon-box expense",
                                    ),
                                    TransactionType::Transfer => (
                                        format_idr(trx.amount),
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
                                                    }
                                                    span { "•" }
                                                    span { "{trx.date}" }
                                                    span { "•" }
                                                    span { class: "tabular-numbers", "{trx.time}" }
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
}
