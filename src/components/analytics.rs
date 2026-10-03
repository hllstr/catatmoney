use dioxus::prelude::*;
use crate::components::icons::{
    CategoryIcon, IconActivity, IconArrowDownRight, IconArrowUpRight,
    IconPieChart, IconReceipt, IconTrendingDown, IconTrendingUp, IconWallet, WalletIcon,
};
use crate::model::{
    format_idr, get_date_days_ago, get_start_of_this_week, get_today_date,
    Transaction, TransactionType, Wallet,
};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnalyticsTimeframe {
    Today,
    ThisWeek,
    ThisMonth,
    Last30Days,
    ThisYear,
    AllTime,
}

impl AnalyticsTimeframe {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Today => "Hari Ini",
            Self::ThisWeek => "Minggu Ini",
            Self::ThisMonth => "Bulan Ini",
            Self::Last30Days => "30 Hari",
            Self::ThisYear => "Tahun Ini",
            Self::AllTime => "Semua",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreakdownMode {
    Expense,
    Income,
}

const DONUT_COLORS: &[&str] = &[
    "#10b981", // Emerald
    "#f43f5e", // Rose
    "#f59e0b", // Amber
    "#0ea5e9", // Sky
    "#8b5cf6", // Violet
    "#6366f1", // Indigo
    "#ec4899", // Pink
    "#14b8a6", // Teal
    "#f97316", // Orange
    "#64748b", // Slate
];

#[component]
pub fn AnalyticsView(
    transactions: Vec<Transaction>,
    wallets: Vec<Wallet>,
    on_select_trx: EventHandler<Transaction>,
) -> Element {
    let _ = on_select_trx;
    let mut timeframe = use_signal(|| AnalyticsTimeframe::ThisMonth);
    let mut breakdown_mode = use_signal(|| BreakdownMode::Expense);

    let today = get_today_date(); // YYYY-MM-DD
    let current_year = if today.len() >= 4 { &today[0..4] } else { "2026" };
    let current_month_prefix = if today.len() >= 7 { &today[0..7] } else { "2026-10" };
    let start_of_week = get_start_of_this_week();
    let thirty_days_ago = get_date_days_ago(30);

    // Saring transaksi berdasarkan timeframe yang dipilih
    let filtered_transactions: Vec<Transaction> = transactions
        .iter()
        .filter(|t| match *timeframe.read() {
            AnalyticsTimeframe::Today => t.date == today,
            AnalyticsTimeframe::ThisWeek => t.date >= start_of_week && t.date <= today,
            AnalyticsTimeframe::ThisMonth => t.date.starts_with(current_month_prefix),
            AnalyticsTimeframe::Last30Days => t.date >= thirty_days_ago && t.date <= today,
            AnalyticsTimeframe::ThisYear => t.date.starts_with(current_year),
            AnalyticsTimeframe::AllTime => true,
        })
        .cloned()
        .collect();


    // Hitung total pemasukan, pengeluaran, dan arus kas bersih
    let mut total_income = 0.0;
    let mut total_expense = 0.0;
    let mut income_count = 0;
    let mut expense_count = 0;

    for t in &filtered_transactions {
        match t.transaction_type {
            TransactionType::Income => {
                total_income += t.amount;
                income_count += 1;
            }
            TransactionType::Expense => {
                total_expense += t.amount;
                expense_count += 1;
            }
            TransactionType::Transfer => {
                if let Some(fee) = t.admin_fee {
                    if fee > 0.0 {
                        total_expense += fee;
                    }
                }
            }
        }
    }

    let net_cashflow = total_income - total_expense;
    let net_cashflow_str = if net_cashflow >= 0.0 {
        format!("+{}", format_idr(net_cashflow))
    } else {
        format_idr(net_cashflow)
    };
    let net_cashflow_class = if net_cashflow >= 0.0 {
        "text-xl font-bold tabular-numbers text-[var(--positive)]"
    } else {
        "text-xl font-bold tabular-numbers text-[var(--negative)]"
    };
    let savings_rate = if total_income > 0.0 {
        ((total_income - total_expense) / total_income * 100.0).clamp(-100.0, 100.0)
    } else {
        0.0
    };

    // Hitung distribusi kategori berdasarkan breakdown_mode
    let mut category_totals: HashMap<String, f64> = HashMap::new();
    let target_type = match *breakdown_mode.read() {
        BreakdownMode::Expense => TransactionType::Expense,
        BreakdownMode::Income => TransactionType::Income,
    };

    for t in &filtered_transactions {
        if t.transaction_type == target_type {
            *category_totals.entry(t.category.clone()).or_insert(0.0) += t.amount;
        }
    }

    let mut sorted_categories: Vec<(String, f64)> = category_totals.into_iter().collect();
    sorted_categories.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let breakdown_total: f64 = sorted_categories.iter().map(|(_, amt)| *amt).sum();

    // Siapkan data segmen Donut Chart SVG
    // Radius lingkaran r = 70, keliling C = 2 * PI * 70 = 439.82
    let circumference = 439.8229715;
    let mut donut_segments: Vec<(String, f64, f64, String, String, &'static str)> = Vec::new();
    let mut accumulated_offset = 0.0;

    for (idx, (cat, amt)) in sorted_categories.iter().enumerate() {
        let percentage = if breakdown_total > 0.0 {
            (*amt / breakdown_total * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        let slice_length = (percentage / 100.0) * circumference;
        let stroke_dasharray = format!("{:.2} {:.2}", slice_length, circumference);
        let stroke_dashoffset = format!("{:.2}", -accumulated_offset);
        let color = DONUT_COLORS[idx % DONUT_COLORS.len()];

        donut_segments.push((
            cat.clone(),
            *amt,
            percentage,
            stroke_dasharray,
            stroke_dashoffset,
            color,
        ));
        accumulated_offset += slice_length;
    }

    // Siapkan data grafik tren harian / bulanan
    let is_monthly_view = matches!(*timeframe.read(), AnalyticsTimeframe::ThisYear | AnalyticsTimeframe::AllTime);
    let mut trend_map: BTreeMap<String, (f64, f64)> = BTreeMap::new();

    for t in &filtered_transactions {
        let key = if is_monthly_view {
            if t.date.len() >= 7 { t.date[0..7].to_string() } else { t.date.clone() }
        } else {
            t.date.clone()
        };

        let entry = trend_map.entry(key).or_insert((0.0, 0.0));
        match t.transaction_type {
            TransactionType::Income => entry.0 += t.amount,
            TransactionType::Expense => entry.1 += t.amount,
            TransactionType::Transfer => {
                if let Some(fee) = t.admin_fee {
                    if fee > 0.0 {
                        entry.1 += fee;
                    }
                }
            }
        }
    }

    let trend_items: Vec<(String, f64, f64)> = trend_map
        .into_iter()
        .map(|(date, (inc, exp))| (date, inc, exp))
        .collect();
    let max_trend_val = trend_items
        .iter()
        .map(|(_, inc, exp)| inc.max(*exp))
        .fold(100_000.0, f64::max);

    let chart_height = 140.0;
    let chart_width = 560.0;
    let bar_width = if trend_items.len() > 15 { 8.0 } else { 14.0 };
    let col_count = trend_items.len() as f64;
    let col_width = if col_count > 0.0 { chart_width / col_count } else { chart_width };

    // Analisis Arus per Sumber Dana (Wallet Flow)
    let mut wallet_flow: HashMap<String, (f64, f64)> = HashMap::new(); // (masuk, keluar)
    for w in &wallets {
        wallet_flow.insert(w.name.clone(), (0.0, 0.0));
    }

    for t in &filtered_transactions {
        match t.transaction_type {
            TransactionType::Income => {
                let entry = wallet_flow.entry(t.wallet.clone()).or_insert((0.0, 0.0));
                entry.0 += t.amount;
            }
            TransactionType::Expense => {
                let entry = wallet_flow.entry(t.wallet.clone()).or_insert((0.0, 0.0));
                entry.1 += t.amount;
            }
            TransactionType::Transfer => {
                let from_entry = wallet_flow.entry(t.wallet.clone()).or_insert((0.0, 0.0));
                from_entry.1 += t.amount + t.admin_fee.unwrap_or(0.0);

                if let Some(ref dest) = t.to_wallet {
                    let to_entry = wallet_flow.entry(dest.clone()).or_insert((0.0, 0.0));
                    to_entry.0 += t.amount;
                }
            }
        }
    }

    // --- KALKULASI KURVA PORTOFOLIO KAS (CRYPTO-STYLE AREA SPLINE) ---
    let mut all_trx_sorted = transactions.clone();
    all_trx_sorted.sort_by(|a, b| {
        let date_cmp = a.date.cmp(&b.date);
        if date_cmp == std::cmp::Ordering::Equal {
            a.time.cmp(&b.time)
        } else {
            date_cmp
        }
    });

    let initial_all_wallets: f64 = wallets.iter().map(|w| w.initial_balance).sum();

    let (tf_start_date, is_intraday) = match *timeframe.read() {
        AnalyticsTimeframe::Today => (today.clone(), true),
        AnalyticsTimeframe::ThisWeek => (start_of_week.clone(), false),
        AnalyticsTimeframe::ThisMonth => (format!("{}-01", current_month_prefix), false),
        AnalyticsTimeframe::Last30Days => (thirty_days_ago.clone(), false),
        AnalyticsTimeframe::ThisYear => (format!("{}-01-01", current_year), false),
        AnalyticsTimeframe::AllTime => {
            let first_date = all_trx_sorted.first().map(|t| t.date.clone()).unwrap_or_else(|| today.clone());
            (first_date, false)
        }
    };

    let mut running_balance = initial_all_wallets;
    for t in &all_trx_sorted {
        if t.date < tf_start_date {
            match t.transaction_type {
                TransactionType::Income => running_balance += t.amount,
                TransactionType::Expense => running_balance -= t.amount,
                TransactionType::Transfer => {
                    if let Some(fee) = t.admin_fee {
                        running_balance -= fee;
                    }
                }
            }
        }
    }
    let period_opening_balance = running_balance;

    let mut crypto_points: Vec<(String, f64)> = Vec::new();

    if is_intraday {
        crypto_points.push(("Awal".to_string(), period_opening_balance));
        for t in &all_trx_sorted {
            if t.date == today {
                match t.transaction_type {
                    TransactionType::Income => running_balance += t.amount,
                    TransactionType::Expense => running_balance -= t.amount,
                    TransactionType::Transfer => {
                        if let Some(fee) = t.admin_fee {
                            running_balance -= fee;
                        }
                    }
                }
                let time_label = if t.time.len() >= 5 { t.time[0..5].to_string() } else { t.time.clone() };
                crypto_points.push((time_label, running_balance));
            }
        }
        if crypto_points.len() == 1 {
            crypto_points.push(("Sekarang".to_string(), running_balance));
        }
    } else {
        let start_label = if tf_start_date.len() >= 10 {
            format!("{}/{}", &tf_start_date[8..10], &tf_start_date[5..7])
        } else {
            "Awal".to_string()
        };
        crypto_points.push((start_label, period_opening_balance));

        let mut date_groups: BTreeMap<String, Vec<&Transaction>> = BTreeMap::new();
        for t in &all_trx_sorted {
            if t.date >= tf_start_date && t.date <= today {
                date_groups.entry(t.date.clone()).or_default().push(t);
            }
        }

        let mut last_date_in_trx = String::new();
        for (d, trxs) in date_groups {
            for t in trxs {
                match t.transaction_type {
                    TransactionType::Income => running_balance += t.amount,
                    TransactionType::Expense => running_balance -= t.amount,
                    TransactionType::Transfer => {
                        if let Some(fee) = t.admin_fee {
                            running_balance -= fee;
                        }
                    }
                }
            }
            let label = if d.len() >= 10 {
                format!("{}/{}", &d[8..10], &d[5..7])
            } else {
                d.clone()
            };
            last_date_in_trx = d;
            crypto_points.push((label, running_balance));
        }

        if last_date_in_trx != today {
            let today_label = if today.len() >= 10 {
                format!("{}/{}", &today[8..10], &today[5..7])
            } else {
                "Hari Ini".to_string()
            };
            crypto_points.push((today_label, running_balance));
        }
    }

    if crypto_points.len() < 2 {
        crypto_points.push(("Kini".to_string(), running_balance));
    }

    let period_start_bal = crypto_points.first().map(|p| p.1).unwrap_or(0.0);
    let period_current_bal = crypto_points.last().map(|p| p.1).unwrap_or(0.0);
    let period_delta = period_current_bal - period_start_bal;
    let period_pct = if period_start_bal.abs() > 0.001 {
        (period_delta / period_start_bal.abs() * 100.0).clamp(-999.0, 999.0)
    } else {
        0.0
    };
    let is_bullish = period_delta >= 0.0;
    let line_color = if is_bullish { "#10b981" } else { "#f43f5e" };
    let glow_class = if is_bullish { "crypto-line-glow-bullish" } else { "crypto-line-glow-bearish" };

    let mut min_val = f64::MAX;
    let mut max_val = f64::MIN;
    let mut ath_idx = 0;
    let mut atl_idx = 0;

    for (i, (_, val)) in crypto_points.iter().enumerate() {
        if *val > max_val {
            max_val = *val;
            ath_idx = i;
        }
        if *val < min_val {
            min_val = *val;
            atl_idx = i;
        }
    }
    if min_val == f64::MAX { min_val = 0.0; }
    if max_val == f64::MIN { max_val = 100_000.0; }

    let spread = max_val - min_val;
    let buffer = if spread > 0.0 { spread * 0.15 } else { 50_000.0 };
    let scale_min = (min_val - buffer).max(0.0);
    let scale_max = max_val + buffer;
    let scale_range = (scale_max - scale_min).max(10_000.0);

    let pad_x = 28.0;
    let pad_top = 26.0;
    let pad_bottom = 34.0;
    let svg_w = 680.0;
    let svg_h = 200.0;
    let plot_w = svg_w - 2.0 * pad_x;
    let plot_h = svg_h - pad_top - pad_bottom;
    let baseline_y = pad_top + plot_h;

    let n_pts = crypto_points.len();
    let coords: Vec<(f64, f64)> = crypto_points
        .iter()
        .enumerate()
        .map(|(i, (_, val))| {
            let x = if n_pts > 1 {
                pad_x + (i as f64 / (n_pts - 1) as f64) * plot_w
            } else {
                pad_x + plot_w / 2.0
            };
            let y = pad_top + plot_h * (1.0 - ((*val - scale_min) / scale_range).clamp(0.0, 1.0));
            (x, y)
        })
        .collect();

    let mut line_path = String::new();
    if !coords.is_empty() {
        line_path.push_str(&format!("M {:.1} {:.1}", coords[0].0, coords[0].1));
        for i in 0..coords.len() - 1 {
            let p0 = coords[i];
            let p1 = coords[i + 1];
            let mid_x = (p0.0 + p1.0) / 2.0;
            line_path.push_str(&format!(
                " C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}",
                mid_x, p0.1, mid_x, p1.1, p1.0, p1.1
            ));
        }
    }

    let area_path = if !coords.is_empty() {
        let first = coords[0];
        let last = coords[coords.len() - 1];
        format!(
            "{} L {:.1} {:.1} L {:.1} {:.1} Z",
            line_path, last.0, baseline_y, first.0, baseline_y
        )
    } else {
        String::new()
    };

    let ath_coord = coords.get(ath_idx).cloned().unwrap_or((pad_x, pad_top));
    let atl_coord = coords.get(atl_idx).cloned().unwrap_or((pad_x, baseline_y));
    let last_coord = coords.last().cloned().unwrap_or((pad_x, pad_top));
    let last_idx = n_pts.saturating_sub(1);

    let val_75 = scale_min + 0.75 * scale_range;
    let val_50 = scale_min + 0.50 * scale_range;
    let val_25 = scale_min + 0.25 * scale_range;
    let y_75 = pad_top + plot_h * 0.25;
    let y_50 = pad_top + plot_h * 0.50;
    let y_25 = pad_top + plot_h * 0.75;

    let mut x_labels: Vec<(f64, String)> = Vec::new();
    if n_pts <= 6 {
        for (i, (lbl, _)) in crypto_points.iter().enumerate() {
            x_labels.push((coords[i].0, lbl.clone()));
        }
    } else {
        let step = (n_pts as f64 / 4.0).max(1.0);
        for k in 0..5 {
            let idx = ((k as f64 * step).round() as usize).min(n_pts - 1);
            x_labels.push((coords[idx].0, crypto_points[idx].0.clone()));
        }
        x_labels.dedup_by(|a, b| (a.0 - b.0).abs() < 35.0);
    }

    rsx! {
        div { class: "analytics-container",
            // Header Halaman & Selector Rentang Waktu
            div { class: "analytics-header-section mb-6 flex flex-col md:flex-row md:items-center justify-between gap-4",
                div {
                    h2 { class: "text-xl font-bold tracking-tight text-[var(--text-primary)]", "Analitik Keuangan" }
                    p { class: "text-xs text-[var(--text-secondary)] mt-0.5",
                        "Distribusi pengeluaran, tren arus kas, dan rasio kesehatan tabungan."
                    }
                }

                // Timeframe Selector Pill (Scrollable horizontal di mobile)
                div { class: "analytics-timeframe-selector inline-flex p-1 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)]",
                    for tf in [
                        AnalyticsTimeframe::Today,
                        AnalyticsTimeframe::ThisWeek,
                        AnalyticsTimeframe::ThisMonth,
                        AnalyticsTimeframe::Last30Days,
                        AnalyticsTimeframe::ThisYear,
                        AnalyticsTimeframe::AllTime,
                    ] {
                        button {
                            r#type: "button",
                            key: "{tf.as_str()}",
                            class: if *timeframe.read() == tf { "timeframe-pill active" } else { "timeframe-pill" },
                            onclick: move |_| timeframe.set(tf),
                            "{tf.as_str()}"
                        }
                    }
                }
            }

            // Konten Analitik dengan Animasi Halus saat Berpindah Rentang Waktu
            div {
                class: "analytics-content-fade",
                key: "{timeframe.read().as_str()}",

                // 4 Executive Financial Health KPI Cards
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6",
                    // Kartu 1: Arus Kas Bersih (Net Cashflow)
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Arus Kas Bersih" }
                            if net_cashflow >= 0.0 {
                                span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--positive)] bg-[var(--positive-bg)] px-2 py-0.5 rounded-full border border-[var(--positive-border)]",
                                    IconTrendingUp { size: "12" }
                                    "Surplus"
                                }
                            } else {
                                span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--negative)] bg-[var(--negative-bg)] px-2 py-0.5 rounded-full border border-[var(--negative-border)]",
                                    IconTrendingDown { size: "12" }
                                    "Defisit"
                                }
                            }
                        }
                        div { class: "{net_cashflow_class}",
                            "{net_cashflow_str}"
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                            "Total pemasukan dikurangi total pengeluaran periode ini."
                        }
                    }

                    // Kartu 2: Rasio Tabungan (Savings Rate)
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Tingkat Tabungan" }
                            span { class: "text-xs font-bold tabular-numbers text-[var(--text-primary)]",
                                "{savings_rate:.1}%"
                            }
                        }
                        div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-2 overflow-hidden my-1.5",
                            div {
                                class: if savings_rate >= 20.0 { "bg-[var(--positive)] h-full rounded-full transition-all" } else if savings_rate > 0.0 { "bg-[var(--amber-500)] h-full rounded-full transition-all" } else { "bg-[var(--negative)] h-full rounded-full transition-all" },
                                style: "width: {savings_rate.max(0.0)}%;",
                            }
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1",
                            if savings_rate >= 30.0 {
                                "Kondisi tabungan prima di atas target 30%."
                            } else if savings_rate > 0.0 {
                                "Masih menghasilkan surplus positif."
                            } else {
                                "Pengeluaran melebihi total pemasukan."
                            }
                        }
                    }

                    // Kartu 3: Total Pemasukan
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Total Pemasukan" }
                            IconArrowUpRight { size: "14" }
                        }
                        div { class: "text-xl font-bold tabular-numbers text-[var(--positive)]",
                            "{format_idr(total_income)}"
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                            "{income_count} kali transaksi masuk"
                        }
                    }

                    // Kartu 4: Total Pengeluaran
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Total Pengeluaran" }
                            IconArrowDownRight { size: "14" }
                        }
                        div { class: "text-xl font-bold tabular-numbers text-[var(--negative)]",
                            "{format_idr(total_expense)}"
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                            "{expense_count} kali transaksi keluar"
                        }
                    }
                }

                // Card Kurva Portofolio Likuiditas Kas (Crypto / TradingView Style Area Spline)
                div { class: "crypto-chart-panel mb-6",
                    // Header: Judul & Metrik Utama ala TradingView / Coinbase
                    div { class: "flex flex-col sm:flex-row sm:items-center justify-between gap-3 mb-4",
                        div {
                            div { class: "flex items-center gap-2",
                                span { class: "w-7 h-7 rounded-lg bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)]",
                                    IconActivity { size: "15" }
                                }
                                div {
                                    h3 { class: "text-sm font-bold text-[var(--text-primary)]", "Kurva Likuiditas & Portofolio Kas" }
                                    p { class: "text-[11px] text-[var(--text-muted)]", "Pergerakan saldo akumulatif berjalan sepanjang periode" }
                                }
                            }
                        }
                        div { class: "flex items-center gap-3 self-start sm:self-auto",
                            div { class: "text-left sm:text-right",
                                div { class: "text-lg font-bold tabular-numbers text-[var(--text-primary)]",
                                    "{format_idr(period_current_bal)}"
                                }
                                div { class: "flex items-center gap-1.5 text-xs font-semibold tabular-numbers",
                                    if is_bullish {
                                        span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--positive)] bg-[var(--positive-bg)] px-2 py-0.5 rounded-full border border-[var(--positive-border)]",
                                            IconTrendingUp { size: "11" }
                                            "+{format_idr(period_delta)} (+{period_pct:.1}%)"
                                        }
                                    } else {
                                        span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--negative)] bg-[var(--negative-bg)] px-2 py-0.5 rounded-full border border-[var(--negative-border)]",
                                            IconTrendingDown { size: "11" }
                                            "{format_idr(period_delta)} ({period_pct:.1}%)"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // SVG Area Curve
                    div { class: "svg-chart-wrapper my-1 overflow-x-auto",
                        svg {
                            view_box: "0 0 680 200",
                            class: "w-full h-auto overflow-visible min-w-[500px]",
                            defs {
                                linearGradient {
                                    id: "cryptoAreaGrad",
                                    x1: "0",
                                    y1: "0",
                                    x2: "0",
                                    y2: "1",
                                    stop { offset: "0%", stop_color: "{line_color}", stop_opacity: "0.32" }
                                    stop { offset: "75%", stop_color: "{line_color}", stop_opacity: "0.04" }
                                    stop { offset: "100%", stop_color: "{line_color}", stop_opacity: "0.0" }
                                }
                            }

                            // Gridlines Horizontal & Labels
                            line { x1: "{pad_x}", y1: "{y_75}", x2: "{680.0 - pad_x}", y2: "{y_75}", class: "crypto-gridline" }
                            text { x: "{680.0 - pad_x}", y: "{y_75 - 4.0}", text_anchor: "end", class: "fill-[var(--text-muted)] text-[9px] tabular-numbers", "{format_idr(val_75)}" }

                            line { x1: "{pad_x}", y1: "{y_50}", x2: "{680.0 - pad_x}", y2: "{y_50}", class: "crypto-gridline" }
                            text { x: "{680.0 - pad_x}", y: "{y_50 - 4.0}", text_anchor: "end", class: "fill-[var(--text-muted)] text-[9px] tabular-numbers", "{format_idr(val_50)}" }

                            line { x1: "{pad_x}", y1: "{y_25}", x2: "{680.0 - pad_x}", y2: "{y_25}", class: "crypto-gridline" }
                            text { x: "{680.0 - pad_x}", y: "{y_25 - 4.0}", text_anchor: "end", class: "fill-[var(--text-muted)] text-[9px] tabular-numbers", "{format_idr(val_25)}" }

                            // Baseline
                            line { x1: "{pad_x}", y1: "{baseline_y}", x2: "{680.0 - pad_x}", y2: "{baseline_y}", class: "crypto-baseline" }

                            // Area Path
                            path {
                                d: "{area_path}",
                                fill: "url(#cryptoAreaGrad)",
                            }

                            // Main Line Path
                            path {
                                d: "{line_path}",
                                fill: "none",
                                stroke: "{line_color}",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "{glow_class}",
                            }

                            // ATH Marker (Titik Tertinggi)
                            if ath_idx != last_idx || coords.len() > 2 {
                                g {
                                    circle { cx: "{ath_coord.0}", cy: "{ath_coord.1}", r: "4.5", fill: "var(--positive)", stroke: "var(--bg-card)", stroke_width: "2" }
                                    text {
                                        x: "{ath_coord.0}",
                                        y: "{ath_coord.1 - 9.0}",
                                        text_anchor: "middle",
                                        class: "fill-[var(--positive)] text-[9px] font-bold",
                                        "ATH"
                                    }
                                }
                            }

                            // ATL Marker (Titik Terendah)
                            if atl_idx != ath_idx && (atl_idx != last_idx || coords.len() > 2) {
                                g {
                                    circle { cx: "{atl_coord.0}", cy: "{atl_coord.1}", r: "4.5", fill: "var(--negative)", stroke: "var(--bg-card)", stroke_width: "2" }
                                    text {
                                        x: "{atl_coord.0}",
                                        y: "{atl_coord.1 + 16.0}",
                                        text_anchor: "middle",
                                        class: "fill-[var(--negative)] text-[9px] font-bold",
                                        "ATL"
                                    }
                                }
                            }

                            // Titik Terakhir (Live Pulse Indicator)
                            g {
                                circle {
                                    cx: "{last_coord.0}",
                                    cy: "{last_coord.1}",
                                    r: "7",
                                    fill: "{line_color}",
                                    class: "crypto-pulse-glow",
                                }
                                circle {
                                    cx: "{last_coord.0}",
                                    cy: "{last_coord.1}",
                                    r: "4",
                                    fill: "{line_color}",
                                    stroke: "var(--bg-card)",
                                    stroke_width: "1.5",
                                }
                            }

                            // Label Sumbu X
                            for (label_x, label_text) in x_labels.iter() {
                                text {
                                    key: "{label_text}",
                                    x: "{label_x}",
                                    y: "{baseline_y + 18.0}",
                                    text_anchor: "middle",
                                    class: "fill-[var(--text-muted)] text-[10px]",
                                    "{label_text}"
                                }
                            }
                        }
                    }

                    // Footer Stats Ringkas
                    div { class: "grid grid-cols-2 sm:grid-cols-4 gap-3 pt-3 mt-3 border-t border-[var(--border-subtle)] text-xs",
                        div {
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Saldo Awal Periode" }
                            span { class: "font-semibold tabular-numbers text-[var(--text-primary)]", "{format_idr(period_start_bal)}" }
                        }
                        div {
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Puncak Tertinggi (ATH)" }
                            span { class: "font-semibold tabular-numbers text-[var(--positive)]", "{format_idr(max_val)}" }
                        }
                        div {
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Dasar Terendah (ATL)" }
                            span { class: "font-semibold tabular-numbers text-[var(--negative)]", "{format_idr(min_val)}" }
                        }
                        div {
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Rentang Fluktuasi" }
                            span { class: "font-semibold tabular-numbers text-[var(--text-secondary)]", "{format_idr(spread)}" }
                        }
                    }
                }

                // Grid Bagian Tengah: SVG Donut Chart Kategori & Tren Arus Kas
                div { class: "grid grid-cols-1 lg:grid-cols-12 gap-6 mb-6",

                // Kolom Kiri (5 kolom): Donut Chart & Breakdown Alokasi
                div { class: "lg:col-span-5 p-5 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col",
                    div { class: "flex items-center justify-between mb-4",
                        h3 { class: "text-sm font-bold text-[var(--text-primary)] flex items-center gap-2",
                            IconPieChart { size: "16" }
                            "Alokasi Kategori"
                        }

                        // Toggle Mode: Pengeluaran vs Pemasukan
                        div { class: "inline-flex p-0.5 rounded-lg bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)]",
                            button {
                                r#type: "button",
                                class: if *breakdown_mode.read() == BreakdownMode::Expense { "timeframe-pill active text-[11px] py-1 px-2.5" } else { "timeframe-pill text-[11px] py-1 px-2.5" },
                                onclick: move |_| breakdown_mode.set(BreakdownMode::Expense),
                                "Pengeluaran"
                            }
                            button {
                                r#type: "button",
                                class: if *breakdown_mode.read() == BreakdownMode::Income { "timeframe-pill active text-[11px] py-1 px-2.5" } else { "timeframe-pill text-[11px] py-1 px-2.5" },
                                onclick: move |_| breakdown_mode.set(BreakdownMode::Income),
                                "Pemasukan"
                            }
                        }
                    }

                    if sorted_categories.is_empty() {
                        div { class: "py-12 flex flex-col items-center justify-center text-center",
                            div { class: "w-10 h-10 rounded-xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-muted)] mb-3",
                                IconReceipt { size: "20" }
                            }
                            p { class: "text-xs font-semibold text-[var(--text-secondary)]", "Belum ada transaksi pada periode ini." }
                            p { class: "text-[11px] text-[var(--text-muted)] mt-1", "Data grafik akan otomatis terbentuk setelah transaksi dicatat." }
                        }
                    } else {
                        div { class: "flex flex-col items-center",
                            // SVG Donut Ring Presisi
                            div { class: "relative w-48 h-48 my-2 flex items-center justify-center",
                                svg {
                                    view_box: "0 0 200 200",
                                    class: "w-full h-full transform -rotate-90 overflow-visible",
                                    // Lingkaran Latar Belakang (Track)
                                    circle {
                                        cx: "100",
                                        cy: "100",
                                        r: "70",
                                        fill: "transparent",
                                        stroke: "var(--border-subtle)",
                                        stroke_width: "16",
                                    }
                                    // Segmen Donut Berwarna
                                    for (_, _, _, s_array, s_offset, color) in donut_segments.iter() {
                                        circle {
                                            key: "{s_offset}",
                                            cx: "100",
                                            cy: "100",
                                            r: "70",
                                            fill: "transparent",
                                            stroke: "{color}",
                                            stroke_width: "16",
                                            stroke_dasharray: "{s_array}",
                                            stroke_dashoffset: "{s_offset}",
                                            stroke_linecap: "round",
                                            class: "donut-ring-circle",
                                        }
                                    }
                                }
                                // Teks di Pusat Donut
                                div { class: "absolute inset-0 flex flex-col items-center justify-center text-center pointer-events-none p-2",
                                    span { class: "text-[10px] uppercase font-semibold text-[var(--text-muted)] tracking-wider",
                                        if *breakdown_mode.read() == BreakdownMode::Expense { "Pengeluaran" } else { "Pemasukan" }
                                    }
                                    span { class: "text-sm font-bold text-[var(--text-primary)] tabular-numbers mt-0.5",
                                        "{format_idr(breakdown_total)}"
                                    }
                                }
                            }

                            // Daftar Rincian Kategori Terurut
                            div { class: "w-full mt-4 space-y-2.5 max-h-60 overflow-y-auto pr-1",
                                for (cat, amt, pct, _, _, color) in donut_segments.iter() {
                                    div { class: "p-2 rounded-lg bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex flex-col gap-1.5",
                                        key: "{cat}",
                                        div { class: "flex items-center justify-between text-xs",
                                            div { class: "flex items-center gap-2",
                                                span {
                                                    class: "w-2.5 h-2.5 rounded-full shrink-0",
                                                    style: "background-color: {color};",
                                                }
                                                CategoryIcon { category: cat.clone() }
                                                span { class: "font-semibold text-[var(--text-primary)] truncate max-w-[140px]", "{cat}" }
                                            }
                                            div { class: "text-right tabular-numbers font-medium text-[var(--text-primary)]",
                                                "{format_idr(*amt)} "
                                                span { class: "text-[10px] text-[var(--text-muted)] font-normal", "({pct:.1}%)" }
                                            }
                                        }
                                        // Mini Progress Bar
                                        div { class: "w-full bg-[var(--bg-app)] rounded-full h-1.5 overflow-hidden",
                                            div {
                                                class: "h-full rounded-full transition-all duration-300",
                                                style: "width: {pct}%; background-color: {color};",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Kolom Kanan (7 kolom): Grafik Batang Arus Kas & Tren Arus Masuk vs Keluar
                div { class: "lg:col-span-7 p-5 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                    div {
                        div { class: "flex items-center justify-between mb-2",
                            h3 { class: "text-sm font-bold text-[var(--text-primary)] flex items-center gap-2",
                                IconTrendingUp { size: "16" }
                                "Tren Arus Kas (Pemasukan vs Pengeluaran)"
                            }
                            div { class: "flex items-center gap-3 text-xs",
                                div { class: "flex items-center gap-1.5",
                                    span { class: "w-2 h-2 rounded-xs bg-[var(--positive)]" }
                                    span { class: "text-[11px] text-[var(--text-secondary)] font-medium", "Pemasukan" }
                                }
                                div { class: "flex items-center gap-1.5",
                                    span { class: "w-2 h-2 rounded-xs bg-[var(--negative)]" }
                                    span { class: "text-[11px] text-[var(--text-secondary)] font-medium", "Pengeluaran" }
                                }
                            }
                        }
                        p { class: "text-xs text-[var(--text-muted)] mb-4",
                            if is_monthly_view { "Distribusi transaksi agregat per bulan." } else { "Distribusi harian kronologis." }
                        }

                        // SVG Arus Kas Responsif
                        div { class: "svg-chart-wrapper my-2 overflow-x-auto",
                            svg {
                                view_box: "0 0 {chart_width} {chart_height + 35.0}",
                                class: "w-full h-auto overflow-visible min-w-[420px]",

                                // Gridline panduan
                                line { x1: "0", y1: "20", x2: "{chart_width}", y2: "20", class: "chart-gridline" }
                                line { x1: "0", y1: "{chart_height / 2.0}", x2: "{chart_width}", y2: "{chart_height / 2.0}", class: "chart-gridline" }
                                line { x1: "0", y1: "{chart_height}", x2: "{chart_width}", y2: "{chart_height}", class: "chart-baseline" }

                                // Batang-batang grafik
                                for (i, (date, inc, exp)) in trend_items.iter().enumerate() {
                                    {
                                        let center_x = (i as f64 * col_width) + (col_width / 2.0);
                                        let inc_h = if max_trend_val > 0.0 { (*inc / max_trend_val * (chart_height - 25.0)).max(2.0) } else { 2.0 };
                                        let exp_h = if max_trend_val > 0.0 { (*exp / max_trend_val * (chart_height - 25.0)).max(2.0) } else { 2.0 };
                                        let inc_y = chart_height - inc_h;
                                        let exp_y = chart_height - exp_h;

                                        let label = if date.len() >= 10 {
                                            format!("{}/{}", &date[8..10], &date[5..7])
                                        } else if date.len() >= 7 {
                                            date[5..7].to_string()
                                        } else {
                                            date.clone()
                                        };

                                        rsx! {
                                            g { key: "{date}", class: "chart-group",
                                                // Batang Pemasukan (Hijau)
                                                rect {
                                                    x: "{center_x - bar_width - 1.0}",
                                                    y: "{inc_y}",
                                                    width: "{bar_width}",
                                                    height: "{inc_h}",
                                                    rx: "2.5",
                                                    class: "chart-bar income",
                                                }
                                                // Batang Pengeluaran (Merah)
                                                rect {
                                                    x: "{center_x + 1.0}",
                                                    y: "{exp_y}",
                                                    width: "{bar_width}",
                                                    height: "{exp_h}",
                                                    rx: "2.5",
                                                    class: "chart-bar expense",
                                                }
                                                // Label Sumbu X
                                                text {
                                                    x: "{center_x}",
                                                    y: "{chart_height + 20.0}",
                                                    text_anchor: "middle",
                                                    class: "chart-date-label text-[10px]",
                                                    "{label}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex items-center justify-between text-[11px] text-[var(--text-muted)] border-t border-[var(--border-subtle)] pt-3 mt-2",
                        span { "Skala Maks: {format_idr(max_trend_val)}" }
                        span { "Total Titik Data: {trend_items.len()}" }
                    }
                }
            }

            // Bagian Bawah: Matriks Performa Aliran Kas per Sumber Dana (Wallet Flow)
            div { class: "p-5 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)]",
                div { class: "flex items-center justify-between mb-4",
                    h3 { class: "text-sm font-bold text-[var(--text-primary)] flex items-center gap-2",
                        IconWallet { size: "16" }
                        "Aliran Kas per Sumber Dana & Dompet"
                    }
                    span { class: "text-xs text-[var(--text-muted)]", "Arus dana masuk vs keluar per akun" }
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for w in wallets.iter() {
                        {
                            let (inflow, outflow) = wallet_flow.get(&w.name).cloned().unwrap_or((0.0, 0.0));
                            let net_wallet = inflow - outflow;
                            let net_wallet_str = if net_wallet >= 0.0 {
                                format!("+{}", format_idr(net_wallet))
                            } else {
                                format_idr(net_wallet)
                            };
                            let net_wallet_class = if net_wallet >= 0.0 {
                                "text-xs font-bold tabular-numbers text-[var(--positive)]"
                            } else {
                                "text-xs font-bold tabular-numbers text-[var(--negative)]"
                            };

                            rsx! {
                                div { class: "p-3.5 rounded-xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex flex-col justify-between",
                                    key: "{w.id}",
                                    div { class: "flex items-center justify-between mb-2",
                                        div { class: "flex items-center gap-2",
                                            div { class: "w-7 h-7 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)]",
                                                WalletIcon { wallet_type: w.wallet_type, size: "14" }
                                            }
                                            div {
                                                span { class: "text-xs font-bold text-[var(--text-primary)] block", "{w.name}" }
                                                span { class: "text-[10px] text-[var(--text-muted)] block", "{w.wallet_type.as_str()}" }
                                            }
                                        }
                                        div { class: "{net_wallet_class}",
                                            "{net_wallet_str}"
                                        }
                                    }

                                    div { class: "grid grid-cols-2 gap-2 text-[11px] pt-2 border-t border-[var(--border-subtle)]",
                                        div {
                                            span { class: "text-[10px] text-[var(--text-muted)] block", "Masuk (+)" }
                                            span { class: "font-semibold tabular-numbers text-[var(--positive)]", "{format_idr(inflow)}" }
                                        }
                                        div { class: "text-right",
                                            span { class: "text-[10px] text-[var(--text-muted)] block", "Keluar (-)" }
                                            span { class: "font-semibold tabular-numbers text-[var(--negative)]", "{format_idr(outflow)}" }
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


