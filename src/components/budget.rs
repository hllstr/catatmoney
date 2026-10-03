use dioxus::prelude::*;
use crate::components::icons::{
    CategoryIcon, IconAlertCircle, IconCheck, IconEdit, IconPlus, IconSparkles,
    IconTarget, IconTrash, IconTrendingDown, IconTrendingUp,
};
use crate::model::{
    format_idr, generate_id, get_default_budgets, get_month_days_info, get_today_date,
    parse_input_idr, CategoryBudget, Transaction, TransactionType, UserCategories,
};
use std::collections::HashMap;

#[component]
pub fn BudgetView(
    transactions: Vec<Transaction>,
    categories: UserCategories,
    budgets: Vec<CategoryBudget>,
    on_update_budgets: EventHandler<Vec<CategoryBudget>>,
) -> Element {
    let mut is_modal_open = use_signal(|| false);
    let mut editing_budget = use_signal(|| None::<CategoryBudget>);
    let mut budget_to_delete = use_signal(|| None::<String>);

    let today = get_today_date(); // YYYY-MM-DD
    let current_month_prefix = if today.len() >= 7 { &today[0..7] } else { "2026-10" };
    let (current_day, total_days_in_month) = get_month_days_info();
    let remaining_days = total_days_in_month.saturating_sub(current_day) + 1;
    let month_progress_pct = (current_day as f64 / total_days_in_month as f64 * 100.0).clamp(0.0, 100.0);

    // Hitung realisasi pengeluaran per kategori untuk bulan berjalan
    let mut category_spent: HashMap<String, f64> = HashMap::new();
    for t in &transactions {
        if t.transaction_type == TransactionType::Expense && t.date.starts_with(current_month_prefix) {
            *category_spent.entry(t.category.clone()).or_insert(0.0) += t.amount;
        }
    }

    // Metrik Agregat Anggaran
    let total_budget: f64 = budgets.iter().map(|b| b.monthly_limit).sum();
    let total_budgeted_spent: f64 = budgets
        .iter()
        .map(|b| category_spent.get(&b.category).cloned().unwrap_or(0.0))
        .sum();
    let total_remaining: f64 = total_budget - total_budgeted_spent;
    let overall_usage_pct = if total_budget > 0.0 {
        (total_budgeted_spent / total_budget * 100.0).clamp(0.0, 999.0)
    } else {
        0.0
    };
    let daily_safe_spend = if total_remaining > 0.0 {
        total_remaining / remaining_days as f64
    } else {
        0.0
    };

    // Evaluasi Pacing (Perjalanan Hari vs Pemakaian Anggaran)
    let pacing_diff = overall_usage_pct - month_progress_pct;
    let (pacing_label, pacing_badge_class, pacing_desc) = if pacing_diff <= -5.0 {
        (
            "Sangat Hemat",
            "text-[var(--positive)] bg-[var(--positive-bg)] border-[var(--positive-border)]",
            "Laju pemakaian anggaran lebih hemat daripada hari kalender yang berjalan.",
        )
    } else if pacing_diff <= 10.0 {
        (
            "Terkendali",
            "text-[var(--text-secondary)] bg-[var(--bg-surface-elevated)] border-[var(--border-subtle)]",
            "Laju pengeluaran proporsional seimbang dengan sisa hari bulan ini.",
        )
    } else {
        (
            "Laju Cepat",
            "text-[var(--negative)] bg-[var(--negative-bg)] border-[var(--negative-border)]",
            "Laju belanja melampaui proporsi kalender. Batasi pos belanja non-esensial.",
        )
    };

    // Handler Tambah/Update Anggaran
    let handle_save_budget = {
        let budgets = budgets.clone();
        move |b: CategoryBudget| {
            let mut list = budgets.clone();
            if let Some(pos) = list.iter().position(|item| item.id == b.id) {
                list[pos] = b;
            } else {
                list.push(b);
            }
            on_update_budgets.call(list);
            is_modal_open.set(false);
            editing_budget.set(None);
        }
    };

    // Handler Terapkan Preset Rekomendasi
    let handle_apply_presets = move |_| {
        on_update_budgets.call(get_default_budgets());
    };


    rsx! {
        div { class: "analytics-container",
            // Header Halaman
            div { class: "analytics-header-section mb-6 flex flex-col sm:flex-row sm:items-center justify-between gap-4",
                div {
                    h2 { class: "text-xl font-bold tracking-tight text-[var(--text-primary)] flex items-center gap-2.5",
                        span { class: "w-8 h-8 rounded-xl bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)]",
                            IconTarget { size: "18" }
                        }
                        "Anggaran & Batas Belanja"
                    }
                    p { class: "text-xs text-[var(--text-secondary)] mt-0.5",
                        "Kendalikan pos pengeluaran bulanan dan pantau pagu belanja harian agar tidak boncos."
                    }
                }

                div { class: "flex items-center gap-2",
                    if budgets.is_empty() {
                        button {
                            r#type: "button",
                            class: "btn-secondary text-xs flex items-center gap-1.5 py-1.5 px-3",
                            onclick: handle_apply_presets,
                            IconSparkles { size: "13" }
                            span { "Gunakan Preset" }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "btn-primary text-xs flex items-center gap-1.5 py-2 px-3.5",
                        onclick: move |_| {
                            editing_budget.set(None);
                            is_modal_open.set(true);
                        },
                        IconPlus { size: "14" }
                        span { "Tambah Anggaran" }
                    }
                }
            }

            // 4 Executive Budget Health Cards
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6",
                // Kartu 1: Total Anggaran Bulanan
                div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                    div { class: "flex items-center justify-between mb-2",
                        span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Total Anggaran" }
                        IconTarget { size: "14" }
                    }
                    div { class: "text-xl font-bold tabular-numbers text-[var(--text-primary)]",
                        "{format_idr(total_budget)}"
                    }
                    p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                        "{budgets.len()} pos kategori dialokasikan"
                    }
                }

                // Kartu 2: Realisasi Terpakai
                div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                    div { class: "flex items-center justify-between mb-2",
                        span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Realisasi Belanja" }
                        span { class: "text-xs font-bold tabular-numbers text-[var(--text-primary)]",
                            "{overall_usage_pct:.1}%"
                        }
                    }
                    div { class: "text-xl font-bold tabular-numbers text-[var(--text-primary)]",
                        "{format_idr(total_budgeted_spent)}"
                    }
                    div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-1.5 overflow-hidden mt-2",
                        div {
                            class: if overall_usage_pct >= 100.0 { "bg-[var(--negative)] h-full rounded-full transition-all" } else if overall_usage_pct >= 75.0 { "bg-[var(--amber-500)] h-full rounded-full transition-all" } else { "bg-[var(--positive)] h-full rounded-full transition-all" },
                            style: "width: {overall_usage_pct.min(100.0)}%;",
                        }
                    }
                }

                // Kartu 3: Sisa Anggaran Tersedia
                div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                    div { class: "flex items-center justify-between mb-2",
                        span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Sisa Anggaran" }
                        if total_remaining >= 0.0 {
                            span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--positive)] bg-[var(--positive-bg)] px-2 py-0.5 rounded-full border border-[var(--positive-border)]",
                                IconTrendingUp { size: "11" }
                                "Aman"
                            }
                        } else {
                            span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--negative)] bg-[var(--negative-bg)] px-2 py-0.5 rounded-full border border-[var(--negative-border)]",
                                IconTrendingDown { size: "11" }
                                "Overbudget"
                            }
                        }
                    }
                    div {
                        class: if total_remaining >= 0.0 { "text-xl font-bold tabular-numbers text-[var(--positive)]" } else { "text-xl font-bold tabular-numbers text-[var(--negative)]" },
                        if total_remaining >= 0.0 {
                            "{format_idr(total_remaining)}"
                        } else {
                            "-{format_idr(total_remaining.abs())}"
                        }
                    }
                    p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                        if total_remaining >= 0.0 { "Kapasitas belanja sisa bulan ini" } else { "Pengeluaran melebihi total pagu" }
                    }
                }

                // Kartu 4: Pagu Belanja Aman Harian (Daily Safe-to-Spend)
                div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                    div { class: "flex items-center justify-between mb-2",
                        span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Pagu Harian Aman" }
                        span { class: "text-[10px] text-[var(--text-muted)]", "{remaining_days} hari tersisa" }
                    }
                    div { class: "text-xl font-bold tabular-numbers text-[var(--text-primary)]",
                        "{format_idr(daily_safe_spend)}"
                        span { class: "text-xs text-[var(--text-muted)] font-normal", " /hari" }
                    }
                    p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                        "Maksimal belanja harian agar tidak boncos."
                    }
                }
            }

            // Banner Indikator Pacing Kalender vs Anggaran
            div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] mb-6 flex flex-col md:flex-row md:items-center justify-between gap-4",
                div { class: "flex items-start sm:items-center gap-3",
                    span { class: "inline-flex items-center px-2.5 py-1 rounded-lg border text-xs font-bold shrink-0 {pacing_badge_class}",
                        "{pacing_label}"
                    }
                    div {
                        p { class: "text-xs font-semibold text-[var(--text-primary)]", "{pacing_desc}" }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-0.5",
                            "Hari ke-{current_day} dari {total_days_in_month} hari ({month_progress_pct:.0}% bulan berjalan) vs {overall_usage_pct:.0}% anggaran terpakai."
                        }
                    }
                }

                // Visual Mini Perbandingan Dual Bar
                div { class: "w-full md:w-56 shrink-0 space-y-1.5 text-[11px]",
                    div { class: "flex justify-between text-[10px] text-[var(--text-muted)]",
                        span { "Perjalanan Waktu" }
                        span { class: "tabular-numbers", "{month_progress_pct:.0}%" }
                    }
                    div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-1 overflow-hidden",
                        div { class: "bg-[var(--text-muted)] h-full rounded-full", style: "width: {month_progress_pct}%;" }
                    }
                    div { class: "flex justify-between text-[10px] text-[var(--text-muted)] pt-0.5",
                        span { "Anggaran Terpakai" }
                        span { class: "tabular-numbers", "{overall_usage_pct:.0}%" }
                    }
                    div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-1 overflow-hidden",
                        div {
                            class: if overall_usage_pct > 100.0 { "bg-[var(--negative)] h-full rounded-full" } else if overall_usage_pct > 75.0 { "bg-[var(--amber-500)] h-full rounded-full" } else { "bg-[var(--positive)] h-full rounded-full" },
                            style: "width: {overall_usage_pct.min(100.0)}%;",
                        }
                    }
                }
            }

            // Daftar Kartu Anggaran per Kategori
            div { class: "space-y-4 mb-8",
                div { class: "flex items-center justify-between",
                    h3 { class: "text-sm font-bold text-[var(--text-primary)]", "Rincian Anggaran Kategori" }
                    span { class: "text-xs text-[var(--text-muted)]", "Batas bulanan vs realisasi belanja" }
                }

                if budgets.is_empty() {
                    div { class: "py-16 p-6 rounded-2xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col items-center justify-center text-center",
                        div { class: "w-12 h-12 rounded-2xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-muted)] mb-3",
                            IconTarget { size: "24" }
                        }
                        h4 { class: "text-sm font-bold text-[var(--text-primary)]", "Belum Ada Anggaran yang Dibuat" }
                        p { class: "text-xs text-[var(--text-muted)] max-w-sm mt-1 mb-5",
                            "Tentukan batas belanja untuk kategori favorit Anda seperti Makanan, Belanja, atau Tagihan untuk menjaga kestabilan finansial."
                        }
                        div { class: "flex flex-wrap items-center justify-center gap-2",
                            button {
                                r#type: "button",
                                class: "btn-secondary text-xs flex items-center gap-1.5 py-2 px-3.5",
                                onclick: handle_apply_presets,
                                IconSparkles { size: "13" }
                                span { "Gunakan Preset Rekomendasi" }
                            }
                            button {
                                r#type: "button",
                                class: "btn-primary text-xs flex items-center gap-1.5 py-2 px-3.5",
                                onclick: move |_| {
                                    editing_budget.set(None);
                                    is_modal_open.set(true);
                                },
                                IconPlus { size: "14" }
                                span { "Buat Anggaran Sendiri" }
                            }
                        }
                    }
                } else {
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        for b in budgets.iter() {
                            {
                                let spent = category_spent.get(&b.category).cloned().unwrap_or(0.0);
                                let limit = b.monthly_limit;
                                let remaining = limit - spent;
                                let usage_pct = if limit > 0.0 { spent / limit * 100.0 } else { 0.0 };
                                let daily_cat_remaining = if remaining > 0.0 { remaining / remaining_days as f64 } else { 0.0 };
                                let is_overbudget = spent > limit;
                                let is_warning = !is_overbudget && usage_pct >= 75.0;

                                let bar_color_class = if is_overbudget {
                                    "bg-[var(--negative)]"
                                } else if is_warning {
                                    "bg-[var(--amber-500)]"
                                } else {
                                    "bg-[var(--positive)]"
                                };

                                let b_id = b.id.clone();
                                let b_clone = b.clone();

                                rsx! {
                                    div {
                                        key: "{b.id}",
                                        class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between transition-all hover:border-[var(--border-color)]",

                                        // Baris Atas: Ikon Kategori, Judul, & Tombol Aksi
                                        div { class: "flex items-center justify-between mb-3",
                                            div { class: "flex items-center gap-2.5",
                                                div { class: "w-9 h-9 rounded-lg bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)] shrink-0",
                                                    CategoryIcon { category: b.category.clone() }
                                                }
                                                div {
                                                    h4 { class: "text-sm font-bold text-[var(--text-primary)]", "{b.category}" }
                                                    div { class: "flex items-center gap-1.5 mt-0.5",
                                                        if is_overbudget {
                                                            span { class: "inline-flex items-center gap-1 text-[10px] font-semibold text-[var(--negative)]",
                                                                IconAlertCircle { size: "11" }
                                                                "Overbudget"
                                                            }
                                                        } else if is_warning {
                                                            span { class: "inline-flex items-center gap-1 text-[10px] font-semibold text-[var(--amber-500)]",
                                                                IconAlertCircle { size: "11" }
                                                                "Hampir Penuh"
                                                            }
                                                        } else {
                                                            span { class: "inline-flex items-center gap-1 text-[10px] font-semibold text-[var(--positive)]",
                                                                IconCheck { size: "11" }
                                                                "Terkendali"
                                                            }
                                                        }
                                                        span { class: "text-[10px] text-[var(--text-muted)]", "• {usage_pct:.0}% terpakai" }
                                                    }
                                                }
                                            }

                                            // Tombol Edit & Hapus
                                            div { class: "flex items-center gap-1",
                                                button {
                                                    r#type: "button",
                                                    class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] text-[var(--text-muted)] hover:text-[var(--text-primary)] flex items-center justify-center transition-colors",
                                                    title: "Ubah Batas Anggaran",
                                                    onclick: move |_| {
                                                        editing_budget.set(Some(b_clone.clone()));
                                                        is_modal_open.set(true);
                                                    },
                                                    IconEdit { size: "13" }
                                                }
                                                button {
                                                    r#type: "button",
                                                    class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] text-[var(--text-muted)] hover:text-[var(--negative)] flex items-center justify-center transition-colors",
                                                    title: "Hapus Anggaran",
                                                    onclick: move |_| {
                                                        budget_to_delete.set(Some(b_id.clone()));
                                                    },
                                                    IconTrash { size: "13" }
                                                }
                                            }
                                        }

                                        // Baris Metrik Nominal: Terpakai vs Batas
                                        div { class: "flex items-baseline justify-between text-xs my-1",
                                            div {
                                                span { class: "text-[10px] text-[var(--text-muted)] block", "Terpakai" }
                                                span { class: "text-sm font-bold tabular-numbers text-[var(--text-primary)]",
                                                    "{format_idr(spent)}"
                                                }
                                            }
                                            div { class: "text-right",
                                                span { class: "text-[10px] text-[var(--text-muted)] block", "Batas Bulanan" }
                                                span { class: "text-xs font-semibold tabular-numbers text-[var(--text-secondary)]",
                                                    "{format_idr(limit)}"
                                                }
                                            }
                                        }

                                        // Progress Bar dengan Marker Hari Ini
                                        div { class: "relative w-full bg-[var(--bg-surface-subtle)] rounded-full h-2 overflow-hidden my-2",
                                            div {
                                                class: "h-full rounded-full transition-all duration-300 {bar_color_class}",
                                                style: "width: {usage_pct.min(100.0)}%;",
                                            }
                                        }

                                        // Baris Keterangan Sisa & Pagu Harian
                                        div { class: "flex items-center justify-between text-[11px] pt-2 border-t border-[var(--border-subtle)] text-[var(--text-muted)]",
                                            if remaining >= 0.0 {
                                                span { "Sisa: "
                                                    strong { class: "text-[var(--text-primary)] tabular-numbers font-medium", "{format_idr(remaining)}" }
                                                }
                                            } else {
                                                span { class: "text-[var(--negative)] font-medium tabular-numbers",
                                                    "Lebih {format_idr(remaining.abs())}"
                                                }
                                            }
                                            if remaining > 0.0 {
                                                span { class: "tabular-numbers", "Pagu: {format_idr(daily_cat_remaining)}/hari" }
                                            } else {
                                                span { class: "text-[10px] text-[var(--negative)]", "Batas terlampaui" }
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

        // Modal Input / Ubah Anggaran
        if *is_modal_open.read() {
            BudgetModal {
                is_open: true,
                categories: categories.expense.clone(),
                existing_budgets: budgets.clone(),
                editing_budget: editing_budget.read().clone(),
                on_close: move |_| {
                    is_modal_open.set(false);
                    editing_budget.set(None);
                },
                on_save: handle_save_budget,
            }
        }

        // Dialog Konfirmasi Hapus Anggaran
        if let Some(ref target_id) = *budget_to_delete.read() {
            {
                let id_to_remove = target_id.clone();
                let budget_name = budgets
                    .iter()
                    .find(|b| b.id == id_to_remove)
                    .map(|b| b.category.clone())
                    .unwrap_or_else(|| "Anggaran".to_string());

                rsx! {
                    div { class: "modal-backdrop",
                        div { class: "modal-dialog max-w-sm",
                            div { class: "modal-header",
                                h3 { class: "modal-title", "Hapus Anggaran" }
                            }
                            div { class: "py-3 text-xs text-[var(--text-secondary)] leading-relaxed",
                                "Apakah Anda yakin ingin menghapus alokasi anggaran untuk kategori "
                                strong { class: "text-[var(--text-primary)]", "{budget_name}" }
                                "? Catatan transaksi Anda tidak akan terpengaruh."
                            }
                            div { class: "modal-actions",
                                button {
                                    r#type: "button",
                                    class: "btn-secondary text-xs",
                                    onclick: move |_| budget_to_delete.set(None),
                                    "Batal"
                                }
                                button {
                                    r#type: "button",
                                    class: "btn-danger-outline text-xs py-2 px-3",
                                    onclick: {
                                        let id = id_to_remove.clone();
                                        let budgets = budgets.clone();
                                        move |_| {
                                            let mut list = budgets.clone();
                                            list.retain(|b| b.id != id);
                                            on_update_budgets.call(list);
                                            budget_to_delete.set(None);
                                        }
                                    },
                                    "Hapus Anggaran"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BudgetModal(
    is_open: bool,
    categories: Vec<String>,
    existing_budgets: Vec<CategoryBudget>,
    editing_budget: Option<CategoryBudget>,
    on_close: EventHandler<()>,
    on_save: EventHandler<CategoryBudget>,
) -> Element {
    let _ = is_open;
    let is_editing = editing_budget.is_some();
    let initial_cat = editing_budget
        .as_ref()
        .map(|b| b.category.clone())
        .unwrap_or_else(|| {
            // Pilih kategori pertama yang belum dianggarkan
            categories
                .iter()
                .find(|c| !existing_budgets.iter().any(|b| b.category == **c))
                .cloned()
                .unwrap_or_else(|| categories.first().cloned().unwrap_or_else(|| "Makanan & Minuman".to_string()))
        });

    let (initial_amount, initial_formatted) = if let Some(ref b) = editing_budget {
        parse_input_idr(&b.monthly_limit.round().to_string())
    } else {
        parse_input_idr("1500000")
    };

    let mut selected_category = use_signal(|| initial_cat);
    let mut amount_val = use_signal(|| initial_amount);
    let mut amount_input = use_signal(|| initial_formatted);
    let mut error_msg = use_signal(|| None::<String>);

    let handle_amount_change = move |evt: FormEvent| {
        let raw = evt.value();
        let (val, formatted) = parse_input_idr(&raw);
        amount_val.set(val);
        amount_input.set(formatted);
        error_msg.set(None);
    };

    let handle_submit = move |_| {
        if *amount_val.read() <= 0.0 {
            error_msg.set(Some("Batas anggaran harus lebih besar dari Rp 0.".to_string()));
            return;
        }

        let cat = selected_category.read().clone();
        // Cegah kategori ganda jika menambah baru
        if !is_editing && existing_budgets.iter().any(|b| b.category == cat) {
            error_msg.set(Some(format!("Kategori \"{}\" sudah memiliki anggaran aktif.", cat)));
            return;
        }

        let id = editing_budget
            .as_ref()
            .map(|b| b.id.clone())
            .unwrap_or_else(generate_id);

        let new_budget = CategoryBudget {
            id,
            category: cat,
            monthly_limit: *amount_val.read(),
        };

        on_save.call(new_budget);
    };

    rsx! {
        div { class: "modal-backdrop",
            div { class: "modal-dialog max-w-md",
                div { class: "modal-header flex items-center justify-between pb-3 border-b border-[var(--border-subtle)]",
                    h3 { class: "modal-title text-base font-bold text-[var(--text-primary)]",
                        if is_editing { "Ubah Batas Anggaran" } else { "Alokasi Anggaran Baru" }
                    }
                    button {
                        r#type: "button",
                        class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] flex items-center justify-center text-[var(--text-muted)]",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                div { class: "py-4 space-y-4",
                    // Pilihan Kategori Pengeluaran
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Kategori Pengeluaran" }
                        if is_editing {
                            div { class: "p-2.5 rounded-lg bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center gap-2 text-xs font-bold text-[var(--text-primary)]",
                                CategoryIcon { category: selected_category.read().clone() }
                                span { "{selected_category.read()}" }
                            }
                        } else {
                            select {
                                class: "field-select w-full",
                                value: "{selected_category.read()}",
                                onchange: move |evt| selected_category.set(evt.value()),
                                for cat in categories.iter() {
                                    option { value: "{cat}", "{cat}" }
                                }
                            }
                        }
                    }

                    // Input Nominal Batas Bulanan
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Batas Anggaran Bulanan (Rp)" }
                        div { class: "relative",
                            span { class: "absolute left-3 top-1/2 -translate-y-1/2 text-xs font-bold text-[var(--text-muted)]", "Rp" }
                            input {
                                r#type: "text",
                                inputmode: "numeric",
                                class: "field-input w-full pl-9 tabular-numbers font-bold text-base",
                                placeholder: "0",
                                value: "{amount_input.read()}",
                                oninput: handle_amount_change,
                            }
                        }

                        // Preset Chips
                        div { class: "flex flex-wrap items-center gap-1.5 mt-2",
                            for (preset_val, preset_label) in [(500_000, "500 rb"), (1_000_000, "1 jt"), (2_500_000, "2.5 jt"), (5_000_000, "5 jt")] {
                                button {
                                    r#type: "button",
                                    class: "preset-pill text-[11px] py-1 px-2.5",
                                    onclick: move |_| {
                                        let (val, formatted) = parse_input_idr(&preset_val.to_string());
                                        amount_val.set(val);
                                        amount_input.set(formatted);
                                        error_msg.set(None);
                                    },
                                    "{preset_label}"
                                }
                            }
                        }
                    }

                    if let Some(ref err) = *error_msg.read() {
                        div { class: "p-2.5 rounded-lg bg-[var(--negative-bg)] border border-[var(--negative-border)] text-xs text-[var(--negative)] font-medium flex items-center gap-1.5",
                            IconAlertCircle { size: "14" }
                            span { "{err}" }
                        }
                    }
                }

                div { class: "modal-actions pt-3 border-t border-[var(--border-subtle)] flex items-center justify-end gap-2",
                    button {
                        r#type: "button",
                        class: "btn-secondary text-xs py-2 px-4",
                        onclick: move |_| on_close.call(()),
                        "Batal"
                    }
                    button {
                        r#type: "button",
                        class: "btn-primary text-xs py-2 px-4",
                        onclick: handle_submit,
                        if is_editing { "Simpan Perubahan" } else { "Buat Anggaran" }
                    }
                }
            }
        }
    }
}
