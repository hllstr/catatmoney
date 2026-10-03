use dioxus::prelude::*;
use crate::components::icons::{
    CategoryIcon, IconAlertCircle, IconArrowDownRight, IconArrowUpRight, IconAward,
    IconCalendar, IconCheck, IconCoins, IconEdit, IconFlag, IconHistory,
    IconMinus, IconPiggyBank, IconPlus, IconSparkles, IconTarget, IconTrash, IconX,
};
use crate::model::{
    calculate_days_remaining, calculate_monthly_savings_needed, format_idr,
    generate_id, get_current_time_hm, get_today_date, parse_input_idr,
    SavingsEntryType, SavingsGoal, SavingsLogEntry, SAVINGS_CATEGORIES,
};

#[component]
pub fn SavingsView(
    savings_goals: Vec<SavingsGoal>,
    savings_logs: Vec<SavingsLogEntry>,
    on_update_goals: EventHandler<Vec<SavingsGoal>>,
    on_update_logs: EventHandler<Vec<SavingsLogEntry>>,
) -> Element {
    let mut active_filter = use_signal(|| "active".to_string()); // "active" | "completed"
    let mut is_goal_modal_open = use_signal(|| false);
    let mut editing_goal = use_signal(|| None::<SavingsGoal>);
    let mut action_goal = use_signal(|| None::<(SavingsGoal, SavingsEntryType)>);
    let mut history_goal = use_signal(|| None::<SavingsGoal>);
    let mut goal_to_delete = use_signal(|| None::<String>);

    // Metrik Agregat Tabungan
    let total_target: f64 = savings_goals.iter().map(|g| g.target_amount).sum();
    let total_saved: f64 = savings_goals.iter().map(|g| g.current_amount).sum();
    let total_remaining: f64 = (total_target - total_saved).max(0.0);
    let overall_pct = if total_target > 0.0 {
        (total_saved / total_target * 100.0).clamp(0.0, 999.0)
    } else {
        0.0
    };

    let active_goals: Vec<SavingsGoal> = savings_goals
        .iter()
        .filter(|g| g.current_amount < g.target_amount)
        .cloned()
        .collect();

    let completed_goals: Vec<SavingsGoal> = savings_goals
        .iter()
        .filter(|g| g.current_amount >= g.target_amount)
        .cloned()
        .collect();

    let filtered_goals: Vec<SavingsGoal> = if *active_filter.read() == "active" {
        active_goals.clone()
    } else {
        completed_goals.clone()
    };

    // Handler Simpan / Update Target
    let handle_save_goal = {
        let goals = savings_goals.clone();
        move |saved_goal: SavingsGoal| {
            let mut list = goals.clone();
            if let Some(pos) = list.iter().position(|g| g.id == saved_goal.id) {
                list[pos] = saved_goal;
            } else {
                list.push(saved_goal);
            }
            on_update_goals.call(list);
            is_goal_modal_open.set(false);
            editing_goal.set(None);
        }
    };

    // Handler Eksekusi Setor / Tarik
    let handle_execute_action = {
        let goals = savings_goals.clone();
        let logs = savings_logs.clone();
        move |(goal_id, action_type, amount, notes): (String, SavingsEntryType, f64, String)| {
            let mut g_list = goals.clone();
            if let Some(pos) = g_list.iter().position(|g| g.id == goal_id) {
                match action_type {
                    SavingsEntryType::Deposit => {
                        g_list[pos].current_amount += amount;
                    }
                    SavingsEntryType::Withdraw => {
                        g_list[pos].current_amount = (g_list[pos].current_amount - amount).max(0.0);
                    }
                }
                on_update_goals.call(g_list);
            }

            let new_log = SavingsLogEntry {
                id: generate_id(),
                goal_id,
                entry_type: action_type,
                amount,
                date: get_today_date(),
                time: get_current_time_hm(),
                notes,
            };
            let mut l_list = logs.clone();
            l_list.insert(0, new_log);
            on_update_logs.call(l_list);
            action_goal.set(None);
        }
    };

    rsx! {
        div { class: "analytics-container",
            // Header Halaman Tabungan
            div { class: "analytics-header-section mb-6 flex flex-col sm:flex-row sm:items-center justify-between gap-4",
                div {
                    h2 { class: "text-xl font-bold tracking-tight text-[var(--text-primary)] flex items-center gap-2.5",
                        span { class: "w-8 h-8 rounded-xl bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)]",
                            IconPiggyBank { size: "18" }
                        }
                        "Target Tabungan & Finansial Impian"
                    }
                    p { class: "text-xs text-[var(--text-secondary)] mt-0.5",
                        "Wujudkan impian finansial secara terukur dengan pelacak progres dan kalkulator setoran cerdas."
                    }
                }

                div { class: "flex items-center gap-2",
                    button {
                        r#type: "button",
                        class: "btn-primary text-xs flex items-center gap-1.5 py-2 px-3.5",
                        onclick: move |_| {
                            editing_goal.set(None);
                            is_goal_modal_open.set(true);
                        },
                        IconPlus { size: "14" }
                        span { "Tambah Target" }
                    }
                }
            }

            // 4 Executive KPI Cards (Hanya jika sudah ada target)
            if !savings_goals.is_empty() {
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6",
                    // Kartu 1: Total Target Impian
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Total Target Impian" }
                            IconTarget { size: "14" }
                        }
                        div { class: "text-xl font-bold tabular-numbers text-[var(--text-primary)]",
                            "{format_idr(total_target)}"
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                            "{savings_goals.len()} impian direncanakan"
                        }
                    }

                    // Kartu 2: Total Dana Terkumpul
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Total Terkumpul" }
                            span { class: "text-xs font-bold tabular-numbers text-[var(--positive)]",
                                "{overall_pct:.1}%"
                            }
                        }
                        div { class: "text-xl font-bold tabular-numbers text-[var(--positive)]",
                            "{format_idr(total_saved)}"
                        }
                        div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-1.5 overflow-hidden mt-2",
                            div {
                                class: "bg-[var(--positive)] h-full rounded-full transition-all duration-300",
                                style: "width: {overall_pct.min(100.0)}%;",
                            }
                        }
                    }

                    // Kartu 3: Progres Rata-rata
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Status Pencapaian" }
                            if completed_goals.len() > 0 {
                                span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--positive)] bg-[var(--positive-bg)] px-2 py-0.5 rounded-full border border-[var(--positive-border)]",
                                    IconAward { size: "11" }
                                    "{completed_goals.len()} Goal Selesai"
                                }
                            } else {
                                span { class: "inline-flex items-center gap-1 text-[11px] font-semibold text-[var(--text-secondary)] bg-[var(--bg-surface-elevated)] px-2 py-0.5 rounded-full border border-[var(--border-subtle)]",
                                    IconCoins { size: "11" }
                                    "Berjalan"
                                }
                            }
                        }
                        div { class: "text-xl font-bold tabular-numbers text-[var(--text-primary)]",
                            "{overall_pct:.0}%"
                            span { class: "text-xs text-[var(--text-muted)] font-normal", " tercapai" }
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                            "{active_goals.len()} target sedang aktif dihimpun"
                        }
                    }

                    // Kartu 4: Sisa Dana Diperlukan
                    div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Kekurangan Dana" }
                            IconFlag { size: "14" }
                        }
                        div { class: "text-xl font-bold tabular-numbers text-[var(--text-primary)]",
                            "{format_idr(total_remaining)}"
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                            "Sisa dana untuk mewujudkan semua impian"
                        }
                    }
                }

                // Filter Tabs: Sedang Berjalan vs Tercapai
                div { class: "flex items-center justify-between mb-4 border-b border-[var(--border-subtle)] pb-2",
                    div { class: "flex items-center gap-2",
                        button {
                            r#type: "button",
                            class: if *active_filter.read() == "active" {
                                "px-3.5 py-1.5 rounded-lg text-xs font-bold bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] text-[var(--text-primary)] transition-all"
                            } else {
                                "px-3.5 py-1.5 rounded-lg text-xs font-medium text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-all"
                            },
                            onclick: move |_| active_filter.set("active".to_string()),
                            "Sedang Berjalan ({active_goals.len()})"
                        }
                        button {
                            r#type: "button",
                            class: if *active_filter.read() == "completed" {
                                "px-3.5 py-1.5 rounded-lg text-xs font-bold bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] text-[var(--text-primary)] transition-all"
                            } else {
                                "px-3.5 py-1.5 rounded-lg text-xs font-medium text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-all"
                            },
                            onclick: move |_| active_filter.set("completed".to_string()),
                            "Tercapai 100% ({completed_goals.len()})"
                        }
                    }
                }
            }

            // Grid Kartu Target Tabungan
            if savings_goals.is_empty() {
                // Empty State Bersih Awal
                div { class: "py-16 p-6 rounded-2xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col items-center justify-center text-center",
                    div { class: "w-12 h-12 rounded-2xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-muted)] mb-3",
                        IconPiggyBank { size: "24" }
                    }
                    h4 { class: "text-sm font-bold text-[var(--text-primary)]", "Belum Ada Target Tabungan" }
                    p { class: "text-xs text-[var(--text-muted)] max-w-sm mt-1 mb-5 leading-relaxed",
                        "Mulai sisihkan dana secara terencana untuk impian Anda, seperti Dana Darurat, Gadget baru, Liburan, hingga DP Rumah."
                    }
                    button {
                        r#type: "button",
                        class: "btn-primary text-xs flex items-center gap-1.5 py-2 px-4",
                        onclick: move |_| {
                            editing_goal.set(None);
                            is_goal_modal_open.set(true);
                        },
                        IconPlus { size: "14" }
                        span { "Buat Target Tabungan Pertama" }
                    }
                }
            } else if filtered_goals.is_empty() {
                // Empty Filter State
                div { class: "py-12 p-6 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col items-center justify-center text-center",
                    div { class: "w-10 h-10 rounded-xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-muted)] mb-2",
                        IconAward { size: "20" }
                    }
                    h4 { class: "text-xs font-bold text-[var(--text-primary)]",
                        if *active_filter.read() == "completed" {
                            "Belum Ada Target yang Selesai"
                        } else {
                            "Semua Target Sedang Selesai"
                        }
                    }
                    p { class: "text-[11px] text-[var(--text-muted)] mt-1",
                        if *active_filter.read() == "completed" {
                            "Terus konsisten menabung! Saat saldo mencapai 100%, kartu impian Anda akan muncul di sini."
                        } else {
                            "Selamat! Seluruh target impian Anda telah terkumpul 100%."
                        }
                    }
                }
            } else {
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    for g in filtered_goals.iter() {
                        {
                            let g_clone = g.clone();
                            let g_id = g.id.clone();
                            let is_completed = g.current_amount >= g.target_amount;
                            let pct = if g.target_amount > 0.0 {
                                (g.current_amount / g.target_amount * 100.0).clamp(0.0, 999.0)
                            } else {
                                0.0
                            };

                            let (days_rem, monthly_needed) = if let Some(ref d) = g.target_date {
                                (
                                    calculate_days_remaining(d),
                                    calculate_monthly_savings_needed(g.target_amount, g.current_amount, d),
                                )
                            } else {
                                (None, None)
                            };

                            rsx! {
                                div { class: "p-4 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-col justify-between hover:border-[var(--border-hover)] transition-all",
                                    div {
                                        // Baris Header Kartu Target
                                        div { class: "flex items-start justify-between gap-3 mb-3",
                                            div { class: "flex items-center gap-2.5",
                                                div { class: "w-9 h-9 rounded-xl bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)] shrink-0",
                                                    CategoryIcon { category: g.category.clone() }
                                                }
                                                div {
                                                    h4 { class: "text-sm font-bold text-[var(--text-primary)] leading-tight", "{g.name}" }
                                                    span { class: "text-[10px] text-[var(--text-muted)] uppercase tracking-wider font-semibold", "{g.category}" }
                                                }
                                            }

                                            // Badge Progres / Status Tercapai
                                            div { class: "flex items-center gap-1.5",
                                                if is_completed {
                                                    span { class: "inline-flex items-center gap-1 text-[11px] font-bold text-[var(--positive)] bg-[var(--positive-bg)] px-2.5 py-0.5 rounded-full border border-[var(--positive-border)]",
                                                        IconAward { size: "12" }
                                                        "Tercapai"
                                                    }
                                                } else {
                                                    span { class: "text-xs font-bold tabular-numbers text-[var(--text-primary)]",
                                                        "{pct:.0}%"
                                                    }
                                                }

                                                // Tombol Edit & Hapus
                                                button {
                                                    r#type: "button",
                                                    class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] text-[var(--text-muted)] hover:text-[var(--text-primary)] flex items-center justify-center transition-colors ml-1",
                                                    title: "Ubah Target",
                                                    onclick: {
                                                        let item = g_clone.clone();
                                                        move |_| {
                                                            editing_goal.set(Some(item.clone()));
                                                            is_goal_modal_open.set(true);
                                                        }
                                                    },
                                                    IconEdit { size: "13" }
                                                }
                                                button {
                                                    r#type: "button",
                                                    class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] text-[var(--text-muted)] hover:text-[var(--negative)] flex items-center justify-center transition-colors",
                                                    title: "Hapus Target",
                                                    onclick: {
                                                        let id = g_id.clone();
                                                        move |_| goal_to_delete.set(Some(id.clone()))
                                                    },
                                                    IconTrash { size: "13" }
                                                }
                                            }
                                        }

                                        // Baris Metrik Nominal: Terkumpul vs Target
                                        div { class: "flex items-baseline justify-between text-xs my-2",
                                            div {
                                                span { class: "text-[10px] text-[var(--text-muted)] block", "Terkumpul" }
                                                span { class: "text-sm font-bold tabular-numbers text-[var(--positive)]",
                                                    "{format_idr(g.current_amount)}"
                                                }
                                            }
                                            div { class: "text-right",
                                                span { class: "text-[10px] text-[var(--text-muted)] block", "Target Impian" }
                                                span { class: "text-xs font-semibold tabular-numbers text-[var(--text-secondary)]",
                                                    "{format_idr(g.target_amount)}"
                                                }
                                            }
                                        }

                                        // Progress Bar
                                        div { class: "w-full bg-[var(--bg-surface-subtle)] rounded-full h-2 overflow-hidden my-2.5",
                                            div {
                                                class: if is_completed {
                                                    "bg-[var(--positive)] h-full rounded-full transition-all duration-300"
                                                } else {
                                                    "bg-[var(--text-primary)] h-full rounded-full transition-all duration-300"
                                                },
                                                style: "width: {pct.min(100.0)}%;",
                                            }
                                        }

                                        // Kotak Informasi Pacing & Tenggat Waktu
                                        div { class: "p-2.5 rounded-lg bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] text-[11px] text-[var(--text-muted)] flex flex-wrap items-center justify-between gap-1.5 my-2",
                                            if let Some(days) = days_rem {
                                                div { class: "flex items-center gap-1.5",
                                                    IconCalendar { size: "12" }
                                                    if days > 0 {
                                                        span { "{days} hari tersisa" }
                                                    } else if days == 0 {
                                                        span { class: "text-[var(--amber-500)] font-semibold", "Deadline Hari Ini" }
                                                    } else {
                                                        span { class: "text-[var(--negative)] font-semibold", "Lewat {days.abs()} hari" }
                                                    }
                                                }
                                            } else {
                                                div { class: "flex items-center gap-1.5",
                                                    IconCalendar { size: "12" }
                                                    span { "Target fleksibel" }
                                                }
                                            }

                                            if !is_completed {
                                                if let Some(m_needed) = monthly_needed {
                                                    div { class: "flex items-center gap-1 text-[var(--text-secondary)]",
                                                        IconSparkles { size: "11" }
                                                        span { "Perlu "
                                                            strong { class: "text-[var(--text-primary)] tabular-numbers font-medium", "{format_idr(m_needed)}" }
                                                            " /bln"
                                                        }
                                                    }
                                                }
                                            } else {
                                                div { class: "flex items-center gap-1 text-[var(--positive)] font-semibold",
                                                    IconCheck { size: "12" }
                                                    span { "Target Lunas Tercapai" }
                                                }
                                            }
                                        }

                                        // Catatan personal jika ada
                                        if !g.notes.is_empty() {
                                            p { class: "text-[11px] text-[var(--text-muted)] italic mt-1 line-clamp-1",
                                                "\"{g.notes}\""
                                            }
                                        }
                                    }

                                    // Baris Tombol Aksi Cepat: Setor, Tarik, Riwayat
                                    div { class: "flex items-center gap-2 pt-3 mt-2 border-t border-[var(--border-subtle)]",
                                        button {
                                            r#type: "button",
                                            class: "btn-primary text-xs py-1.5 px-3 flex-1 flex items-center justify-center gap-1",
                                            onclick: {
                                                let item = g_clone.clone();
                                                move |_| action_goal.set(Some((item.clone(), SavingsEntryType::Deposit)))
                                            },
                                            IconPlus { size: "13" }
                                            span { "Setor Dana" }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "btn-secondary text-xs py-1.5 px-3 flex-1 flex items-center justify-center gap-1",
                                            disabled: g.current_amount <= 0.0,
                                            onclick: {
                                                let item = g_clone.clone();
                                                move |_| action_goal.set(Some((item.clone(), SavingsEntryType::Withdraw)))
                                            },
                                            IconMinus { size: "13" }
                                            span { "Tarik Dana" }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "btn-secondary text-xs py-1.5 px-2.5 flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-primary)]",
                                            title: "Lihat Riwayat Setor/Tarik",
                                            onclick: {
                                                let item = g_clone.clone();
                                                move |_| history_goal.set(Some(item.clone()))
                                            },
                                            IconHistory { size: "13" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Modal Input Target Tabungan Baru / Edit
        if *is_goal_modal_open.read() {
            SavingsGoalModal {
                is_open: true,
                editing_goal: editing_goal.read().clone(),
                on_close: move |_| {
                    is_goal_modal_open.set(false);
                    editing_goal.set(None);
                },
                on_save: handle_save_goal,
            }
        }

        // Modal Setor / Tarik Dana
        if let Some((target_g, action_type)) = action_goal.read().clone() {
            SavingsActionModal {
                goal: target_g,
                action_type,
                on_close: move |_| action_goal.set(None),
                on_submit: handle_execute_action,
            }
        }

        // Modal Riwayat Log Kontribusi
        if let Some(target_g) = history_goal.read().clone() {
            SavingsHistoryModal {
                goal: target_g.clone(),
                logs: savings_logs.iter().filter(|l| l.goal_id == target_g.id).cloned().collect(),
                on_close: move |_| history_goal.set(None),
            }
        }

        // Dialog Konfirmasi Hapus Target Tabungan
        if let Some(ref target_id) = *goal_to_delete.read() {
            {
                let id_to_remove = target_id.clone();
                let goal_name = savings_goals
                    .iter()
                    .find(|g| g.id == id_to_remove)
                    .map(|g| g.name.clone())
                    .unwrap_or_else(|| "Target Tabungan".to_string());

                rsx! {
                    div {
                        class: "modal-backdrop modal-backdrop-center",
                        onclick: move |_| goal_to_delete.set(None),
                        div {
                            class: "modal-dialog max-w-sm",
                            onclick: move |e| e.stop_propagation(),
                            div { class: "modal-header",
                                h3 { class: "modal-title", "Hapus Target Tabungan" }
                            }
                            div { class: "py-3 text-xs text-[var(--text-secondary)] leading-relaxed",
                                "Apakah Anda yakin ingin menghapus target impian "
                                strong { class: "text-[var(--text-primary)]", "\"{goal_name}\"" }
                                "? Log riwayat setoran terkait target ini juga akan dibersihkan."
                            }
                            div { class: "modal-actions",
                                button {
                                    r#type: "button",
                                    class: "btn-secondary text-xs",
                                    onclick: move |_| goal_to_delete.set(None),
                                    "Batal"
                                }
                                button {
                                    r#type: "button",
                                    class: "btn-danger-outline text-xs py-2 px-3",
                                    onclick: {
                                        let id = id_to_remove.clone();
                                        let goals = savings_goals.clone();
                                        let logs = savings_logs.clone();
                                        move |_| {
                                            let mut g_list = goals.clone();
                                            g_list.retain(|g| g.id != id);
                                            on_update_goals.call(g_list);

                                            let mut l_list = logs.clone();
                                            l_list.retain(|l| l.goal_id != id);
                                            on_update_logs.call(l_list);

                                            goal_to_delete.set(None);
                                        }
                                    },
                                    "Hapus Target"
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
fn SavingsGoalModal(
    is_open: bool,
    editing_goal: Option<SavingsGoal>,
    on_close: EventHandler<()>,
    on_save: EventHandler<SavingsGoal>,
) -> Element {
    let _ = is_open;
    let is_editing = editing_goal.is_some();

    let initial_name = editing_goal.as_ref().map(|g| g.name.clone()).unwrap_or_default();
    let initial_category = editing_goal.as_ref().map(|g| g.category.clone()).unwrap_or_else(|| "Dana Darurat".to_string());
    let (initial_target_amount, initial_target_formatted) = if let Some(ref g) = editing_goal {
        parse_input_idr(&g.target_amount.round().to_string())
    } else {
        (0.0, String::new())
    };
    let initial_date = editing_goal.as_ref().and_then(|g| g.target_date.clone()).unwrap_or_default();
    let initial_notes = editing_goal.as_ref().map(|g| g.notes.clone()).unwrap_or_default();

    let mut name_input = use_signal(|| initial_name);
    let mut category_input = use_signal(|| initial_category);
    let mut target_val = use_signal(|| initial_target_amount);
    let mut target_str = use_signal(|| initial_target_formatted);
    let mut target_date_input = use_signal(|| initial_date);
    let mut notes_input = use_signal(|| initial_notes);
    let mut error_msg = use_signal(|| None::<String>);

    let handle_amount_change = move |evt: FormEvent| {
        let raw = evt.value();
        let (val, formatted) = parse_input_idr(&raw);
        target_val.set(val);
        target_str.set(formatted);
        error_msg.set(None);
    };

    let handle_submit = move |_| {
        let n = name_input.read().trim().to_string();
        if n.is_empty() {
            error_msg.set(Some("Nama target impian tidak boleh kosong.".to_string()));
            return;
        }
        if *target_val.read() <= 0.0 {
            error_msg.set(Some("Nominal target harus lebih besar dari Rp 0.".to_string()));
            return;
        }

        let date_opt = {
            let d = target_date_input.read().trim().to_string();
            if d.is_empty() { None } else { Some(d) }
        };

        let id = editing_goal.as_ref().map(|g| g.id.clone()).unwrap_or_else(generate_id);
        let current = editing_goal.as_ref().map(|g| g.current_amount).unwrap_or(0.0);
        let created_at = editing_goal.as_ref().map(|g| g.created_at.clone()).unwrap_or_else(get_today_date);

        let new_goal = SavingsGoal {
            id,
            name: n,
            category: category_input.read().clone(),
            target_amount: *target_val.read(),
            current_amount: current,
            target_date: date_opt,
            created_at,
            notes: notes_input.read().trim().to_string(),
        };

        on_save.call(new_goal);
    };

    rsx! {
        div {
            class: "modal-backdrop modal-backdrop-center",
            onclick: move |_| on_close.call(()),
            div {
                class: "modal-dialog max-w-md",
                onclick: move |e| e.stop_propagation(),
                div { class: "modal-header flex items-center justify-between pb-3 border-b border-[var(--border-subtle)]",
                    h3 { class: "modal-title text-base font-bold text-[var(--text-primary)] flex items-center gap-2",
                        IconPiggyBank { size: "18" }
                        if is_editing { "Ubah Target Tabungan" } else { "Buat Target Tabungan Baru" }
                    }
                    button {
                        r#type: "button",
                        class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors",
                        onclick: move |_| on_close.call(()),
                        IconX { size: "16" }
                    }
                }

                div { class: "py-4 space-y-4",
                    // Nama Target Impian
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Nama Target / Impian" }
                        input {
                            r#type: "text",
                            class: "field-input w-full text-sm",
                            placeholder: "Contoh: Dana Darurat, MacBook Pro, Liburan Jepang",
                            value: "{name_input.read()}",
                            oninput: move |e| {
                                name_input.set(e.value());
                                error_msg.set(None);
                            },
                        }
                    }

                    // Kategori Impian
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Kategori Impian" }
                        select {
                            class: "field-select w-full",
                            value: "{category_input.read()}",
                            onchange: move |e| category_input.set(e.value()),
                            for cat in SAVINGS_CATEGORIES.iter() {
                                option { value: "{cat}", "{cat}" }
                            }
                        }
                    }

                    // Target Nominal (Rp)
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Nominal Target (Rp)" }
                        div { class: "currency-input-box",
                            span { class: "currency-prefix tabular-numbers", "Rp" }
                            input {
                                r#type: "text",
                                inputmode: "numeric",
                                class: "field-input currency-input tabular-numbers",
                                placeholder: "0",
                                value: "{target_str.read()}",
                                oninput: handle_amount_change,
                            }
                        }

                        // Preset Chips
                        div { class: "flex flex-wrap items-center gap-1.5 mt-2",
                            for (preset_val, preset_label) in [(1_000_000, "1 jt"), (5_000_000, "5 jt"), (10_000_000, "10 jt"), (25_000_000, "25 jt")] {
                                button {
                                    r#type: "button",
                                    class: "preset-pill text-[11px] py-1 px-2.5",
                                    onclick: move |_| {
                                        let (val, formatted) = parse_input_idr(&preset_val.to_string());
                                        target_val.set(val);
                                        target_str.set(formatted);
                                        error_msg.set(None);
                                    },
                                    "{preset_label}"
                                }
                            }
                        }
                    }

                    // Tenggat Waktu (Opsional)
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Tenggat Waktu / Deadline (Opsional)" }
                        input {
                            r#type: "date",
                            class: "field-input w-full text-sm",
                            value: "{target_date_input.read()}",
                            oninput: move |e| target_date_input.set(e.value()),
                        }
                        span { class: "text-[10px] text-[var(--text-muted)] mt-1 block",
                            "Sistem akan otomatis menghitung rekomendasi setoran bulanan yang dibutuhkan."
                        }
                    }

                    // Catatan Personal / Motivasi
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Catatan Motivasi (Opsional)" }
                        input {
                            r#type: "text",
                            class: "field-input w-full text-sm",
                            placeholder: "Contoh: Untuk ketenangan finansial 6 bulan ke depan",
                            value: "{notes_input.read()}",
                            oninput: move |e| notes_input.set(e.value()),
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
                        if is_editing { "Simpan Perubahan" } else { "Buat Target" }
                    }
                }
            }
        }
    }
}

#[component]
fn SavingsActionModal(
    goal: SavingsGoal,
    action_type: SavingsEntryType,
    on_close: EventHandler<()>,
    on_submit: EventHandler<(String, SavingsEntryType, f64, String)>,
) -> Element {
    let mut amount_val = use_signal(|| 0.0);
    let mut amount_str = use_signal(String::new);
    let mut notes_input = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);

    let is_deposit = action_type == SavingsEntryType::Deposit;
    let goal_id = goal.id.clone();
    let goal_name = goal.name.clone();
    let current_amt = goal.current_amount;
    let target_amt = goal.target_amount;

    let handle_amount_change = move |evt: FormEvent| {
        let (val, formatted) = parse_input_idr(&evt.value());
        amount_val.set(val);
        amount_str.set(formatted);
        error_msg.set(None);
    };

    let handle_submit = move |_| {
        let val = *amount_val.read();
        if val <= 0.0 {
            error_msg.set(Some("Nominal harus lebih besar dari Rp 0.".to_string()));
            return;
        }

        if !is_deposit && val > current_amt {
            error_msg.set(Some(format!(
                "Nominal penarikan melebihi saldo tabungan saat ini ({})",
                format_idr(current_amt)
            )));
            return;
        }

        let n = notes_input.read().trim().to_string();
        let default_note = if is_deposit { "Setoran tabungan" } else { "Penarikan tabungan" };
        let final_note = if n.is_empty() { default_note.to_string() } else { n };

        on_submit.call((goal_id.clone(), action_type, val, final_note));
    };

    rsx! {
        div {
            class: "modal-backdrop modal-backdrop-center",
            onclick: move |_| on_close.call(()),
            div {
                class: "modal-dialog max-w-md",
                onclick: move |e| e.stop_propagation(),
                div { class: "modal-header flex items-center justify-between pb-3 border-b border-[var(--border-subtle)]",
                    h3 { class: "modal-title text-base font-bold text-[var(--text-primary)] flex items-center gap-2",
                        if is_deposit {
                            IconPlus { size: "16" }
                        } else {
                            IconMinus { size: "16" }
                        }
                        if is_deposit { "Setor Dana Tabungan" } else { "Tarik Dana Tabungan" }
                    }
                    button {
                        r#type: "button",
                        class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors",
                        onclick: move |_| on_close.call(()),
                        IconX { size: "16" }
                    }
                }

                div { class: "py-4 space-y-4",
                    // Informasi Target
                    div { class: "p-3 rounded-xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-between",
                        div {
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Target Impian" }
                            span { class: "text-xs font-bold text-[var(--text-primary)]", "{goal_name}" }
                        }
                        div { class: "text-right",
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Saldo Saat Ini" }
                            span { class: "text-xs font-bold tabular-numbers text-[var(--positive)]",
                                "{format_idr(current_amt)}"
                            }
                            span { class: "text-[10px] text-[var(--text-muted)] tabular-numbers block",
                                "Target: {format_idr(target_amt)}"
                            }
                        }
                    }

                    // Input Nominal Setor / Tarik
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5",
                            if is_deposit { "Jumlah Nominal Setoran (Rp)" } else { "Jumlah Nominal Penarikan (Rp)" }
                        }
                        div { class: "currency-input-box",
                            span { class: "currency-prefix tabular-numbers", "Rp" }
                            input {
                                r#type: "text",
                                inputmode: "numeric",
                                class: "field-input currency-input tabular-numbers",
                                placeholder: "0",
                                value: "{amount_str.read()}",
                                oninput: handle_amount_change,
                            }
                        }

                        // Preset Chips
                        div { class: "flex flex-wrap items-center gap-1.5 mt-2",
                            for (preset_val, preset_label) in [(50_000, "50 rb"), (100_000, "100 rb"), (250_000, "250 rb"), (500_000, "500 rb"), (1_000_000, "1 jt")] {
                                button {
                                    r#type: "button",
                                    class: "preset-pill text-[11px] py-1 px-2.5",
                                    onclick: move |_| {
                                        let (val, formatted) = parse_input_idr(&preset_val.to_string());
                                        amount_val.set(val);
                                        amount_str.set(formatted);
                                        error_msg.set(None);
                                    },
                                    "{preset_label}"
                                }
                            }
                        }
                    }

                    // Catatan Singkat
                    div { class: "field-group",
                        label { class: "block text-xs font-semibold text-[var(--text-secondary)] mb-1.5", "Catatan Mutasi (Opsional)" }
                        input {
                            r#type: "text",
                            class: "field-input w-full text-sm",
                            placeholder: if is_deposit { "Contoh: Sisihan gaji, Bonus freelance, Uang kembalian" } else { "Contoh: Kebutuhan mendesak, Pembayaran bertahap" },
                            value: "{notes_input.read()}",
                            oninput: move |e| notes_input.set(e.value()),
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
                        class: if is_deposit { "btn-primary text-xs py-2 px-4" } else { "btn-secondary text-xs py-2 px-4 font-bold text-[var(--text-primary)]" },
                        onclick: handle_submit,
                        if is_deposit { "Simpan Setoran" } else { "Konfirmasi Tarik" }
                    }
                }
            }
        }
    }
}

#[component]
fn SavingsHistoryModal(
    goal: SavingsGoal,
    logs: Vec<SavingsLogEntry>,
    on_close: EventHandler<()>,
) -> Element {
    let goal_name = goal.name.clone();
    let current_amt = goal.current_amount;
    let target_amt = goal.target_amount;

    rsx! {
        div {
            class: "modal-backdrop modal-backdrop-center",
            onclick: move |_| on_close.call(()),
            div {
                class: "modal-dialog max-w-md",
                onclick: move |e| e.stop_propagation(),
                div { class: "modal-header flex items-center justify-between pb-3 border-b border-[var(--border-subtle)]",
                    div {
                        h3 { class: "modal-title text-base font-bold text-[var(--text-primary)] flex items-center gap-2",
                            IconHistory { size: "16" }
                            "Riwayat Setoran Tabungan"
                        }
                        p { class: "text-xs text-[var(--text-muted)] mt-0.5", "{goal_name}" }
                    }
                    button {
                        r#type: "button",
                        class: "w-7 h-7 rounded-lg hover:bg-[var(--bg-surface-elevated)] flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors",
                        onclick: move |_| on_close.call(()),
                        IconX { size: "16" }
                    }
                }

                div { class: "py-4 space-y-3",
                    // Ringkasan Cepat
                    div { class: "p-3 rounded-xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-between text-xs",
                        div {
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Saldo Terkumpul" }
                            span { class: "font-bold tabular-numbers text-[var(--positive)]", "{format_idr(current_amt)}" }
                        }
                        div { class: "text-right",
                            span { class: "text-[10px] text-[var(--text-muted)] block", "Target Impian" }
                            span { class: "font-semibold tabular-numbers text-[var(--text-secondary)]", "{format_idr(target_amt)}" }
                        }
                    }

                    // Daftar Log Mutasi
                    if logs.is_empty() {
                        div { class: "py-8 text-center",
                            div { class: "w-9 h-9 rounded-xl bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-muted)] mx-auto mb-2",
                                IconCoins { size: "18" }
                            }
                            p { class: "text-xs text-[var(--text-muted)]", "Belum ada catatan setoran atau penarikan pada target ini." }
                        }
                    } else {
                        div { class: "space-y-2 max-h-72 overflow-y-auto pr-1",
                            for l in logs.iter() {
                                {
                                    let is_deposit = l.entry_type == SavingsEntryType::Deposit;
                                    rsx! {
                                        div { class: "p-2.5 rounded-lg bg-[var(--bg-surface-subtle)] border border-[var(--border-subtle)] flex items-center justify-between gap-3 text-xs",
                                            div { class: "flex items-center gap-2",
                                                div {
                                                    class: if is_deposit {
                                                        "w-7 h-7 rounded-lg bg-[var(--positive-bg)] text-[var(--positive)] border border-[var(--positive-border)] flex items-center justify-center shrink-0"
                                                    } else {
                                                        "w-7 h-7 rounded-lg bg-[var(--negative-bg)] text-[var(--negative)] border border-[var(--negative-border)] flex items-center justify-center shrink-0"
                                                    },
                                                    if is_deposit {
                                                        IconArrowDownRight { size: "14" }
                                                    } else {
                                                        IconArrowUpRight { size: "14" }
                                                    }
                                                }
                                                div {
                                                    span { class: "font-semibold text-[var(--text-primary)] block leading-tight", "{l.notes}" }
                                                    span { class: "text-[10px] text-[var(--text-muted)] tabular-numbers",
                                                        "{l.date} • {l.time}"
                                                    }
                                                }
                                            }

                                            div { class: "text-right shrink-0",
                                                span {
                                                    class: if is_deposit {
                                                        "font-bold tabular-numbers text-[var(--positive)]"
                                                    } else {
                                                        "font-bold tabular-numbers text-[var(--negative)]"
                                                    },
                                                    if is_deposit {
                                                        "+{format_idr(l.amount)}"
                                                    } else {
                                                        "-{format_idr(l.amount)}"
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

                div { class: "modal-actions pt-3 border-t border-[var(--border-subtle)] flex items-center justify-end",
                    button {
                        r#type: "button",
                        class: "btn-secondary text-xs py-2 px-4",
                        onclick: move |_| on_close.call(()),
                        "Tutup"
                    }
                }
            }
        }
    }
}
