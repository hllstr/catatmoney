use dioxus::prelude::*;
use std::rc::Rc;
use crate::components::icons::{
    CategoryIcon, IconAlertCircle, IconArrowRight, IconBot, IconCheck, IconEye,
    IconEyeOff, IconKey, IconPaperclip, IconPiggyBank, IconReceipt,
    IconSend, IconSparkles, IconTarget, IconTrash, IconWallet, IconX,
};
use crate::model::{
    calculate_wallet_balance, format_idr, generate_id, get_current_time_hm, get_today_date,
    load_ai_chat_history, save_ai_chat_history, AiActionStatus, AiChatMessage,
    AiMessageRole, AiProposedAction, CategoryBudget, SavingsEntryType, SavingsGoal,
    SavingsLogEntry, Transaction, TransactionType, UserCategories, Wallet, GEMINI_MODEL,
};

#[component]
pub fn AiCopilotView(
    user_name: String,
    gemini_api_key: Option<String>,
    transactions: Vec<Transaction>,
    wallets: Vec<Wallet>,
    categories: UserCategories,
    budgets: Vec<CategoryBudget>,
    savings_goals: Vec<SavingsGoal>,
    savings_logs: Vec<SavingsLogEntry>,
    on_save_api_key: EventHandler<String>,
    on_record_transaction: EventHandler<Transaction>,
    on_update_budgets: EventHandler<Vec<CategoryBudget>>,
    on_update_goals: EventHandler<Vec<SavingsGoal>>,
    on_update_logs: EventHandler<Vec<SavingsLogEntry>>,
    on_go_to_settings: EventHandler<()>,
) -> Element {
    // Chat messages signal, initialized from LocalStorage
    let mut messages = use_signal(load_ai_chat_history);
    let mut input_text = use_signal(String::new);
    let is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    // Image upload state for multimodal receipts
    let mut attached_image_data = use_signal(|| None::<String>);
    let mut attached_image_name = use_signal(|| None::<String>);

    // Onboarding API key input state
    let mut temp_key_input = use_signal(String::new);
    let mut show_key_text = use_signal(|| false);
    let mut is_editing_key = use_signal(|| false);

    let has_key = gemini_api_key.as_ref().map(|k| !k.trim().is_empty()).unwrap_or(false);

    // Context wrapped in Rc for multi-closure usage
    let budgets_rc = Rc::new(budgets);
    let savings_rc = Rc::new(savings_goals);
    let logs_rc = Rc::new(savings_logs);
    let wallets_rc = Rc::new(wallets);
    let trxs_rc = Rc::new(transactions);
    let cats_rc = Rc::new(categories);
    let user_name_rc = Rc::new(user_name);
    let api_key_rc = Rc::new(gemini_api_key);

    // Handler clear chat
    let handle_clear_chat = move |_| {
        messages.set(vec![]);
        save_ai_chat_history(&[]);
        error_msg.set(None);
    };

    rsx! {
        div { class: "ai-copilot-container max-w-4xl mx-auto space-y-4 pb-28",
            // Header Top Bar
            div { class: "p-3 sm:p-4 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-col sm:flex-row sm:items-center justify-between gap-3 shadow-sm",
                div { class: "flex items-start sm:items-center gap-2.5 sm:gap-3 min-w-0",
                    div { class: "w-8 h-8 sm:w-9 sm:h-9 rounded-lg bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--accent)] shrink-0 mt-0.5 sm:mt-0",
                        IconSparkles { size: "18" }
                    }
                    div { class: "min-w-0 flex-1",
                        div { class: "flex items-center gap-2",
                            h2 { class: "text-sm sm:text-base font-bold text-[var(--text-primary)] tracking-tight", "Gemini AI Copilot" }
                            if has_key {
                                span { class: "text-[10px] font-semibold px-2 py-0.5 rounded-full bg-[var(--positive-bg)] text-[var(--positive)] border border-[var(--positive-border)] flex items-center gap-1 font-mono",
                                    span { class: "w-1.5 h-1.5 rounded-full bg-[var(--positive)] animate-pulse shrink-0" }
                                    "Aktif"
                                }
                            }
                        }
                        p { class: "text-[11px] sm:text-xs text-[var(--text-muted)] mt-0.5 hidden xs:block sm:block truncate",
                            "Asisten otonom untuk mencatat transaksi, scan struk, dan mengelola target tabungan."
                        }
                    }
                }

                div { class: "flex items-center justify-end gap-1.5 sm:gap-2 shrink-0 border-t sm:border-t-0 pt-2 sm:pt-0 border-[var(--border-subtle)]",
                    if has_key {
                        button {
                            r#type: "button",
                            title: "Ubah API Key",
                            class: "btn-secondary text-xs px-2.5 py-1.5 flex items-center gap-1.5",
                            onclick: move |_| {
                                let next_v = !*is_editing_key.read();
                                is_editing_key.set(next_v);
                            },
                            IconKey { size: "13" }
                            span { "API Key" }
                        }
                    }
                    if !messages.read().is_empty() {
                        button {
                            r#type: "button",
                            title: "Bersihkan Percakapan",
                            class: "btn-secondary text-xs px-2.5 py-1.5 text-[var(--negative)] hover:bg-[var(--negative-bg)] flex items-center gap-1.5",
                            onclick: handle_clear_chat,
                            IconTrash { size: "13" }
                            span { "Bersihkan" }
                        }
                    }
                }
            }

            // Onboarding / Quick Edit API Key Card
            if !has_key || *is_editing_key.read() {
                div { class: "p-3.5 sm:p-4 rounded-xl border border-[var(--accent)]/30 bg-[var(--bg-card)] space-y-3",
                    div { class: "flex items-start justify-between gap-3",
                        div { class: "flex items-center gap-2.5 min-w-0",
                            span { class: "w-8 h-8 rounded-lg bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--accent)] shrink-0",
                                IconKey { size: "16" }
                            }
                            div { class: "min-w-0",
                                h3 { class: "text-sm font-bold text-[var(--text-primary)]",
                                    if has_key { "Perbarui Google Gemini API Key" } else { "Aktivasi Gemini AI Copilot" }
                                }
                                p { class: "text-xs text-[var(--text-secondary)]",
                                    "Masukkan API Key gratis dari Google AI Studio. Key hanya disimpan di LocalStorage perangkat Anda."
                                }
                            }
                        }
                        if has_key {
                            button {
                                r#type: "button",
                                class: "text-[var(--text-muted)] hover:text-[var(--text-primary)] p-1 shrink-0",
                                onclick: move |_| is_editing_key.set(false),
                                IconX { size: "16" }
                            }
                        }
                    }

                    div { class: "flex flex-col sm:flex-row gap-2 pt-1",
                        div { class: "relative flex-1",
                            input {
                                r#type: if *show_key_text.read() { "text" } else { "password" },
                                class: "field-input w-full pr-10 text-xs font-mono tabular-numbers",
                                placeholder: "Tempelkan API Key (AIzaSy...)",
                                value: "{temp_key_input}",
                                oninput: move |e| temp_key_input.set(e.value()),
                            }
                            button {
                                r#type: "button",
                                class: "absolute right-2.5 top-1/2 -translate-y-1/2 text-[var(--text-muted)] hover:text-[var(--text-primary)]",
                                onclick: move |_| {
                                    let next_v = !*show_key_text.read();
                                    show_key_text.set(next_v);
                                },
                                if *show_key_text.read() {
                                    IconEyeOff { size: "14" }
                                } else {
                                    IconEye { size: "14" }
                                }
                            }
                        }

                        div { class: "flex flex-wrap sm:flex-nowrap items-center gap-2 shrink-0",
                            button {
                                r#type: "button",
                                class: "btn-primary text-xs px-4 py-2 font-semibold flex items-center justify-center gap-1.5 flex-1 sm:flex-initial",
                                onclick: move |_| {
                                    let val = temp_key_input.read().trim().to_string();
                                    if !val.is_empty() {
                                        on_save_api_key.call(val);
                                        temp_key_input.set(String::new());
                                        is_editing_key.set(false);
                                        error_msg.set(None);
                                    }
                                },
                                IconCheck { size: "14" }
                                "Simpan Key"
                            }
                            a {
                                href: "https://aistudio.google.com/app/apikey",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "btn-secondary text-xs px-3 py-2 flex items-center justify-center gap-1 text-[var(--text-secondary)] flex-1 sm:flex-initial",
                                "Dapatkan Key Gratis"
                                IconArrowRight { size: "12" }
                            }
                        }
                    }
                }
            }

            // Error Banner Alert
            if let Some(err) = error_msg.read().as_ref() {
                div { class: "p-3 rounded-xl bg-[var(--negative-bg)] border border-[var(--negative-border)] text-[var(--negative)] flex items-center justify-between text-xs gap-3",
                    div { class: "flex items-center gap-2",
                        IconAlertCircle { size: "16" }
                        span { class: "font-medium", "{err}" }
                    }
                    button {
                        r#type: "button",
                        class: "text-[var(--negative)] hover:opacity-80 p-0.5",
                        onclick: move |_| error_msg.set(None),
                        IconX { size: "14" }
                    }
                }
            }

            // Main Message Feed Container
            div { class: "chat-messages-scroll rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)] p-4 min-h-[380px] max-h-[58vh] overflow-y-auto space-y-4 shadow-inner",
                if messages.read().is_empty() {
                    // Empty state hero with prompt recommendations
                    div { class: "flex flex-col items-center justify-center py-10 text-center px-4",
                        div { class: "w-12 h-12 rounded-2xl bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--accent)] mb-3 shadow-sm",
                            IconBot { size: "24" }
                        }
                        h3 { class: "text-base font-bold text-[var(--text-primary)] mb-1", "Asisten Keuangan Cerdas" }
                        p { class: "text-xs text-[var(--text-muted)] max-w-md leading-relaxed mb-6",
                            "Ketik instruksi untuk mencatat pemasukan, pengeluaran, anggaran, atau unggah foto struk belanja untuk dicatat otomatis."
                        }

                        div { class: "w-full max-w-lg space-y-2 text-left",
                            span { class: "text-[11px] font-semibold text-[var(--text-secondary)] block mb-1.5", "Rekomendasi Coba:" }
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-2",
                                button {
                                    r#type: "button",
                                    class: "p-2.5 rounded-lg bg-[var(--bg-surface-subtle)] hover:bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-all text-left flex items-center justify-between",
                                    onclick: move |_| input_text.set("Makan siang Rp 35.000 bayar via BCA".to_string()),
                                    span { "\"Makan siang Rp 35.000 via BCA\"" }
                                    IconArrowRight { size: "12" }
                                }
                                button {
                                    r#type: "button",
                                    class: "p-2.5 rounded-lg bg-[var(--bg-surface-subtle)] hover:bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-all text-left flex items-center justify-between",
                                    onclick: move |_| input_text.set("Berapa sisa anggaran belanja saya bulan ini?".to_string()),
                                    span { "\"Berapa sisa anggaran belanja bulan ini?\"" }
                                    IconArrowRight { size: "12" }
                                }
                                button {
                                    r#type: "button",
                                    class: "p-2.5 rounded-lg bg-[var(--bg-surface-subtle)] hover:bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-all text-left flex items-center justify-between",
                                    onclick: move |_| input_text.set("Setor Rp 100.000 ke Tabungan Dana Darurat".to_string()),
                                    span { "\"Setor Rp 100.000 ke Dana Darurat\"" }
                                    IconArrowRight { size: "12" }
                                }
                                button {
                                    r#type: "button",
                                    class: "p-2.5 rounded-lg bg-[var(--bg-surface-subtle)] hover:bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] text-xs text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-all text-left flex items-center justify-between",
                                    onclick: move |_| input_text.set("Catat pemasukan freelance Rp 1.500.000 masuk ke BCA".to_string()),
                                    span { "\"Pemasukan freelance Rp 1.5jt ke BCA\"" }
                                    IconArrowRight { size: "12" }
                                }
                            }
                        }
                    }
                } else {
                    for msg in messages.read().iter() {
                        {
                            let is_user = msg.role == AiMessageRole::User;
                            let msg_id = msg.id.clone();
                            rsx! {
                                div {
                                    key: "{msg.id}",
                                    class: if is_user {
                                        "flex justify-end"
                                    } else {
                                        "flex justify-start"
                                    },

                                    div {
                                        class: if is_user {
                                            "max-w-[85%] sm:max-w-[75%] rounded-2xl rounded-tr-sm bg-[var(--bg-surface-elevated)] border border-[var(--border-subtle)] p-3 text-xs text-[var(--text-primary)] shadow-sm space-y-2"
                                        } else {
                                            "max-w-[90%] sm:max-w-[80%] rounded-2xl rounded-tl-sm bg-[var(--bg-app)] border border-[var(--border-subtle)] p-3.5 text-xs text-[var(--text-primary)] shadow-sm space-y-3"
                                        },

                                        // Thumbnail lampiran pengguna jika ada
                                        if let Some(ref img) = msg.attachment_preview {
                                            div { class: "rounded-lg overflow-hidden border border-[var(--border-subtle)] max-h-48 max-w-xs",
                                                img {
                                                    src: "{img}",
                                                    alt: "Lampiran struk",
                                                    class: "w-full h-auto object-cover",
                                                }
                                            }
                                        }

                                        // Isi teks pesan
                                        p { class: "leading-relaxed whitespace-pre-wrap", "{msg.text}" }

                                        // Kartu Konfirmasi Aksi Interaktif (Confirmation Card)
                                        if let Some(ref action) = msg.proposed_action {
                                            {
                                                let status = msg.action_status;
                                                rsx! {
                                                    div { class: "p-3 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-2.5 mt-2",
                                                        // Action Card Header
                                                        div { class: "flex items-center justify-between pb-2 border-b border-[var(--border-subtle)]",
                                                            div { class: "flex items-center gap-1.5 font-bold text-[11px] text-[var(--text-primary)]",
                                                                match action {
                                                                    AiProposedAction::RecordTransaction { .. } => rsx! {
                                                                        IconReceipt { size: "14" }
                                                                        "Rincian Transaksi Baru"
                                                                    },
                                                                    AiProposedAction::DepositSavings { .. } => rsx! {
                                                                        IconPiggyBank { size: "14" }
                                                                        "Rincian Setor Tabungan"
                                                                    },
                                                                    AiProposedAction::WithdrawSavings { .. } => rsx! {
                                                                        IconPiggyBank { size: "14" }
                                                                        "Rincian Tarik Tabungan"
                                                                    },
                                                                    AiProposedAction::SetBudget { .. } => rsx! {
                                                                        IconTarget { size: "14" }
                                                                        "Rincian Alokasi Anggaran"
                                                                    },
                                                                }
                                                            }

                                                            // Status Badge
                                                            match status {
                                                                AiActionStatus::Pending => rsx! {
                                                                    span { class: "text-[10px] font-semibold px-2 py-0.5 rounded bg-[var(--amber-500)]/10 text-[var(--amber-500)] border border-[var(--amber-500)]/20",
                                                                        "Menunggu Konfirmasi"
                                                                    }
                                                                },
                                                                AiActionStatus::Confirmed => rsx! {
                                                                    span { class: "text-[10px] font-semibold px-2 py-0.5 rounded bg-[var(--positive-bg)] text-[var(--positive)] border border-[var(--positive-border)] flex items-center gap-1",
                                                                        IconCheck { size: "11" }
                                                                        "Tersimpan"
                                                                    }
                                                                },
                                                                AiActionStatus::Cancelled => rsx! {
                                                                    span { class: "text-[10px] font-semibold px-2 py-0.5 rounded bg-[var(--bg-surface-elevated)] text-[var(--text-muted)] border border-[var(--border-subtle)]",
                                                                        "Dibatalkan"
                                                                    }
                                                                },
                                                            }
                                                        }

                                                        // Action Card Details Content
                                                        match action {
                                                            AiProposedAction::RecordTransaction { title, amount, transaction_type, category, wallet, date, time, notes, attachment } => rsx! {
                                                                div { class: "grid grid-cols-2 gap-2 text-[11px]",
                                                                    div { class: "col-span-2",
                                                                        span { class: "text-[10px] text-[var(--text-muted)] block", "Judul Transaksi" }
                                                                        span { class: "font-semibold text-[var(--text-primary)] break-words block", "{title}" }
                                                                    }
                                                                    div {
                                                                        span { class: "text-[10px] text-[var(--text-muted)] block", "Nominal" }
                                                                        span {
                                                                            class: match transaction_type {
                                                                                TransactionType::Income => "font-bold tabular-numbers text-[var(--positive)]",
                                                                                TransactionType::Expense => "font-bold tabular-numbers text-[var(--negative)]",
                                                                                TransactionType::Transfer => "font-bold tabular-numbers text-[var(--transfer)]",
                                                                            },
                                                                            "{format_idr(*amount)}"
                                                                        }
                                                                    }
                                                                    div {
                                                                        span { class: "text-[10px] text-[var(--text-muted)] block", "Sumber Dana" }
                                                                        div { class: "flex items-center gap-1 text-[var(--text-primary)] mt-0.5 font-medium truncate",
                                                                            IconWallet { size: "13" }
                                                                            span { class: "truncate", "{wallet}" }
                                                                        }
                                                                    }
                                                                    div { class: "col-span-2 sm:col-span-1",
                                                                        span { class: "text-[10px] text-[var(--text-muted)] block", "Kategori" }
                                                                        div { class: "flex items-center gap-1.5 text-[var(--text-primary)] mt-0.5 font-medium truncate",
                                                                            CategoryIcon { category: category.clone() }
                                                                            span { class: "truncate", "{category}" }
                                                                        }
                                                                    }
                                                                }

                                                                div { class: "text-[10px] text-[var(--text-muted)] pt-1 flex items-center justify-between border-t border-[var(--border-subtle)]",
                                                                    span { class: "tabular-numbers", "{date} • {time} WIB" }
                                                                    if attachment.is_some() {
                                                                        span { class: "text-[var(--accent)] flex items-center gap-1 shrink-0",
                                                                            IconPaperclip { size: "11" }
                                                                            "Ada Struk"
                                                                        }
                                                                    }
                                                                }

                                                                if !notes.is_empty() {
                                                                    p { class: "text-[10px] text-[var(--text-muted)] italic break-words", "\"{notes}\"" }
                                                                }
                                                            },
                                                            AiProposedAction::DepositSavings { goal_name, amount, notes } => rsx! {
                                                                div { class: "space-y-1.5 text-[11px]",
                                                                    div { class: "flex justify-between items-center gap-2",
                                                                        span { class: "text-[var(--text-muted)] shrink-0", "Target Tabungan:" }
                                                                        strong { class: "text-[var(--text-primary)] font-semibold truncate text-right", "{goal_name}" }
                                                                    }
                                                                    div { class: "flex justify-between items-center gap-2",
                                                                        span { class: "text-[var(--text-muted)] shrink-0", "Nominal Setoran:" }
                                                                        strong { class: "tabular-numbers font-bold text-[var(--positive)] shrink-0", "+{format_idr(*amount)}" }
                                                                    }
                                                                    if !notes.is_empty() {
                                                                        p { class: "text-[10px] text-[var(--text-muted)] italic break-words", "\"{notes}\"" }
                                                                    }
                                                                }
                                                            },
                                                            AiProposedAction::WithdrawSavings { goal_name, amount, notes } => rsx! {
                                                                div { class: "space-y-1.5 text-[11px]",
                                                                    div { class: "flex justify-between items-center gap-2",
                                                                        span { class: "text-[var(--text-muted)] shrink-0", "Target Tabungan:" }
                                                                        strong { class: "text-[var(--text-primary)] font-semibold truncate text-right", "{goal_name}" }
                                                                    }
                                                                    div { class: "flex justify-between items-center gap-2",
                                                                        span { class: "text-[var(--text-muted)] shrink-0", "Nominal Penarikan:" }
                                                                        strong { class: "tabular-numbers font-bold text-[var(--negative)] shrink-0", "-{format_idr(*amount)}" }
                                                                    }
                                                                    if !notes.is_empty() {
                                                                        p { class: "text-[10px] text-[var(--text-muted)] italic break-words", "\"{notes}\"" }
                                                                    }
                                                                }
                                                            },
                                                            AiProposedAction::SetBudget { category, monthly_limit } => rsx! {
                                                                div { class: "space-y-1.5 text-[11px]",
                                                                    div { class: "flex justify-between items-center gap-2",
                                                                        span { class: "text-[var(--text-muted)] shrink-0", "Kategori Pengeluaran:" }
                                                                        strong { class: "text-[var(--text-primary)] font-semibold truncate text-right", "{category}" }
                                                                    }
                                                                    div { class: "flex justify-between items-center gap-2",
                                                                        span { class: "text-[var(--text-muted)] shrink-0", "Pagu Anggaran Bulanan:" }
                                                                        strong { class: "tabular-numbers font-bold text-[var(--accent)] shrink-0", "{format_idr(*monthly_limit)}" }
                                                                    }
                                                                }
                                                            },
                                                        }

                                                        // Interactive Buttons for Pending Status
                                                        if status == AiActionStatus::Pending {
                                                            div { class: "flex flex-col sm:flex-row items-stretch sm:items-center gap-2 pt-2 border-t border-[var(--border-subtle)]",
                                                                button {
                                                                    r#type: "button",
                                                                    class: "btn-primary text-xs px-3 py-2 font-semibold flex-1 flex items-center justify-center gap-1.5 bg-[var(--positive)] hover:opacity-90 text-white",
                                                                    onclick: {
                                                                        let id = msg_id.clone();
                                                                        let s_goals = savings_rc.clone();
                                                                        let s_logs = logs_rc.clone();
                                                                        let s_budgets = budgets_rc.clone();
                                                                        move |_| {
                                                                            execute_confirm_action(
                                                                                &id,
                                                                                messages,
                                                                                &s_goals,
                                                                                &s_logs,
                                                                                &s_budgets,
                                                                                &on_record_transaction,
                                                                                &on_update_goals,
                                                                                &on_update_logs,
                                                                                &on_update_budgets,
                                                                            );
                                                                        }
                                                                    },
                                                                    IconCheck { size: "13" }
                                                                    "Konfirmasi & Simpan"
                                                                }
                                                                button {
                                                                    r#type: "button",
                                                                    class: "btn-secondary text-xs px-3 py-2 flex items-center justify-center gap-1 text-[var(--text-muted)] hover:text-[var(--text-primary)]",
                                                                    onclick: {
                                                                        let id = msg_id.clone();
                                                                        move |_| {
                                                                            let mut list = messages.read().clone();
                                                                            if let Some(pos) = list.iter().position(|m| m.id == id) {
                                                                                list[pos].action_status = AiActionStatus::Cancelled;
                                                                                messages.set(list.clone());
                                                                                save_ai_chat_history(&list);
                                                                            }
                                                                        }
                                                                    },
                                                                    IconX { size: "13" }
                                                                    "Batalkan"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Message Timestamp
                                        div { class: "text-[9px] text-[var(--text-muted)] text-right tabular-numbers pt-0.5",
                                            "{msg.timestamp}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Loading Indicator (Minimal 3 Animated Dots)
                if *is_loading.read() {
                    div { class: "flex justify-start",
                        div { class: "rounded-2xl rounded-tl-sm bg-[var(--bg-app)] border border-[var(--border-subtle)] px-4 py-2.5 flex items-center gap-1.5 shadow-sm",
                            span { class: "w-1.5 h-1.5 rounded-full bg-[var(--accent)] animate-bounce" }
                            span { class: "w-1.5 h-1.5 rounded-full bg-[var(--accent)] animate-bounce [animation-delay:0.15s]" }
                            span { class: "w-1.5 h-1.5 rounded-full bg-[var(--accent)] animate-bounce [animation-delay:0.3s]" }
                        }
                    }
                }

                // Scroll anchor
                div { id: "ai-chat-bottom-anchor" }
            }

            // Attached receipt image preview chip
            if let Some(ref name) = attached_image_name.read().as_ref() {
                div { class: "p-2.5 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex items-center justify-between gap-3 text-xs",
                    div { class: "flex items-center gap-2.5 min-w-0",
                        if let Some(ref img_data) = attached_image_data.read().as_ref() {
                            img {
                                src: "{img_data}",
                                alt: "Preview",
                                class: "w-8 h-8 rounded object-cover border border-[var(--border-subtle)] shrink-0",
                            }
                        }
                        div { class: "min-w-0",
                            span { class: "text-[11px] font-semibold text-[var(--text-primary)] truncate block", "{name}" }
                            span { class: "text-[10px] text-[var(--text-muted)]", "Foto struk siap dipindai oleh Gemini" }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "w-6 h-6 rounded-md hover:bg-[var(--bg-surface-elevated)] flex items-center justify-center text-[var(--text-muted)] hover:text-[var(--text-primary)]",
                        onclick: move |_| {
                            attached_image_data.set(None);
                            attached_image_name.set(None);
                        },
                        IconX { size: "14" }
                    }
                }
            }

            // Chat Input Bar Container
            div { class: "p-1.5 sm:p-2 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)] shadow-lg flex items-center gap-1.5 sm:gap-2",
                // Hidden file input for receipts
                input {
                    r#type: "file",
                    id: "ai-receipt-file-input",
                    accept: "image/*",
                    class: "hidden",
                    onchange: move |_e| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            use wasm_bindgen::JsCast;
                            if let Some(window) = web_sys::window() {
                                if let Some(doc) = window.document() {
                                    if let Some(el) = doc.get_element_by_id("ai-receipt-file-input") {
                                        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                                            if let Some(files) = input.files() {
                                                if let Some(file) = files.get(0) {
                                                    let name = file.name();
                                                    attached_image_name.set(Some(name));

                                                    if let Ok(reader) = web_sys::FileReader::new() {
                                                        let reader_clone = reader.clone();
                                                        let mut att_data = attached_image_data;
                                                        let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                                                            if let Ok(val) = reader_clone.result() {
                                                                if let Some(s) = val.as_string() {
                                                                    att_data.set(Some(s));
                                                                }
                                                            }
                                                        }) as Box<dyn FnMut(_)>);
                                                        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                                        onload.forget();
                                                        let _ = reader.read_as_data_url(&file);
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

                // Button Upload Struk / Gambar
                button {
                    r#type: "button",
                    title: "Unggah Foto Struk / Bukti Transfer",
                    class: "w-9 h-9 rounded-lg hover:bg-[var(--bg-surface-elevated)] flex items-center justify-center text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors shrink-0",
                    onclick: move |_| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            if let Some(window) = web_sys::window() {
                                if let Some(doc) = window.document() {
                                    if let Some(el) = doc.get_element_by_id("ai-receipt-file-input") {
                                        if let Ok(html_el) = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(el) {
                                            html_el.click();
                                        }
                                    }
                                }
                            }
                        }
                    },
                    IconPaperclip { size: "18" }
                }

                // Text Input
                input {
                    r#type: "text",
                    class: "flex-1 bg-transparent border-none outline-none text-xs text-[var(--text-primary)] placeholder-[var(--text-muted)] px-2 py-1.5",
                    placeholder: if attached_image_data.read().is_some() { "Ketik instruksi tambahan, atau tekan Kirim untuk scan struk..." } else { "Ketik perintah transaksi atau tanya keuangan..." },
                    value: "{input_text}",
                    disabled: *is_loading.read(),
                    oninput: move |e| input_text.set(e.value()),
                    onkeydown: {
                        let u = user_name_rc.clone();
                        let w = wallets_rc.clone();
                        let t = trxs_rc.clone();
                        let c = cats_rc.clone();
                        let b = budgets_rc.clone();
                        let s = savings_rc.clone();
                        let k = api_key_rc.clone();
                        move |evt: KeyboardEvent| {
                            if evt.key() == Key::Enter && !*is_loading.read() {
                                execute_send_message(
                                    k.as_deref(),
                                    &u,
                                    &w,
                                    &t,
                                    &c,
                                    &b,
                                    &s,
                                    messages,
                                    input_text,
                                    attached_image_data,
                                    attached_image_name,
                                    error_msg,
                                    is_loading,
                                );
                            }
                        }
                    }
                }

                // Send Button
                button {
                    r#type: "button",
                    title: "Kirim Pesan",
                    disabled: *is_loading.read() || (input_text.read().trim().is_empty() && attached_image_data.read().is_none()),
                    class: "w-9 h-9 rounded-lg bg-[var(--text-primary)] text-[var(--bg-app)] hover:opacity-90 disabled:opacity-30 disabled:pointer-events-none flex items-center justify-center transition-all shrink-0",
                    onclick: {
                        let u = user_name_rc.clone();
                        let w = wallets_rc.clone();
                        let t = trxs_rc.clone();
                        let c = cats_rc.clone();
                        let b = budgets_rc.clone();
                        let s = savings_rc.clone();
                        let k = api_key_rc.clone();
                        move |_| {
                            execute_send_message(
                                k.as_deref(),
                                &u,
                                &w,
                                &t,
                                &c,
                                &b,
                                &s,
                                messages,
                                input_text,
                                attached_image_data,
                                attached_image_name,
                                error_msg,
                                is_loading,
                            );
                        }
                    },
                    IconSend { size: "16" }
                }
            }
        }
    }
}

fn scroll_chat_to_bottom() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.get_element_by_id("ai-chat-bottom-anchor") {
                    el.scroll_into_view();
                }
            }
        }
    }
}

fn execute_confirm_action(
    msg_id: &str,
    mut messages: Signal<Vec<AiChatMessage>>,
    savings_goals: &[SavingsGoal],
    savings_logs: &[SavingsLogEntry],
    budgets: &[CategoryBudget],
    on_record_transaction: &EventHandler<Transaction>,
    on_update_goals: &EventHandler<Vec<SavingsGoal>>,
    on_update_logs: &EventHandler<Vec<SavingsLogEntry>>,
    on_update_budgets: &EventHandler<Vec<CategoryBudget>>,
) {
    let mut list = messages.read().clone();
    if let Some(pos) = list.iter().position(|m| m.id == msg_id) {
        let msg = &list[pos];
        if let Some(ref action) = msg.proposed_action {
            match action {
                AiProposedAction::RecordTransaction {
                    title,
                    amount,
                    transaction_type,
                    category,
                    wallet,
                    date,
                    time,
                    notes,
                    attachment,
                } => {
                    let new_trx = Transaction {
                        id: generate_id(),
                        title: title.clone(),
                        amount: *amount,
                        transaction_type: *transaction_type,
                        category: category.clone(),
                        wallet: wallet.clone(),
                        to_wallet: None,
                        admin_fee: None,
                        date: date.clone(),
                        time: time.clone(),
                        notes: notes.clone(),
                        attachment_name: if attachment.is_some() { Some("struk_ai.jpg".to_string()) } else { None },
                        attachment_size: if attachment.is_some() { Some("Scan AI".to_string()) } else { None },
                        attachment_data: attachment.clone(),
                    };
                    on_record_transaction.call(new_trx);
                }
                AiProposedAction::DepositSavings { goal_name, amount, notes } => {
                    let mut goals = savings_goals.to_vec();
                    if let Some(goal) = goals.iter_mut().find(|g| g.name.eq_ignore_ascii_case(goal_name) || g.name.contains(goal_name) || goal_name.contains(&g.name)) {
                        goal.current_amount += *amount;
                        let new_log = SavingsLogEntry {
                            id: generate_id(),
                            goal_id: goal.id.clone(),
                            amount: *amount,
                            entry_type: SavingsEntryType::Deposit,
                            date: get_today_date(),
                            time: get_current_time_hm(),
                            notes: notes.clone(),
                        };
                        let mut logs = savings_logs.to_vec();
                        logs.insert(0, new_log);
                        on_update_goals.call(goals);
                        on_update_logs.call(logs);
                    }
                }
                AiProposedAction::WithdrawSavings { goal_name, amount, notes } => {
                    let mut goals = savings_goals.to_vec();
                    if let Some(goal) = goals.iter_mut().find(|g| g.name.eq_ignore_ascii_case(goal_name) || g.name.contains(goal_name) || goal_name.contains(&g.name)) {
                        goal.current_amount = (goal.current_amount - *amount).max(0.0);
                        let new_log = SavingsLogEntry {
                            id: generate_id(),
                            goal_id: goal.id.clone(),
                            amount: *amount,
                            entry_type: SavingsEntryType::Withdraw,
                            date: get_today_date(),
                            time: get_current_time_hm(),
                            notes: notes.clone(),
                        };
                        let mut logs = savings_logs.to_vec();
                        logs.insert(0, new_log);
                        on_update_goals.call(goals);
                        on_update_logs.call(logs);
                    }
                }
                AiProposedAction::SetBudget { category, monthly_limit } => {
                    let mut b_list = budgets.to_vec();
                    if let Some(pos_b) = b_list.iter().position(|b| b.category.eq_ignore_ascii_case(category)) {
                        b_list[pos_b].monthly_limit = *monthly_limit;
                    } else {
                        b_list.push(CategoryBudget {
                            id: generate_id(),
                            category: category.clone(),
                            monthly_limit: *monthly_limit,
                        });
                    }
                    on_update_budgets.call(b_list);
                }
            }
        }

        // Mark as confirmed
        list[pos].action_status = AiActionStatus::Confirmed;
        messages.set(list.clone());
        save_ai_chat_history(&list);
    }
}

fn execute_send_message(
    api_key_opt: Option<&str>,
    user_name: &str,
    wallets: &[Wallet],
    trxs: &[Transaction],
    cats: &UserCategories,
    budgets: &[CategoryBudget],
    savings: &[SavingsGoal],
    mut messages: Signal<Vec<AiChatMessage>>,
    mut input_text: Signal<String>,
    mut attached_image_data: Signal<Option<String>>,
    mut attached_image_name: Signal<Option<String>>,
    mut error_msg: Signal<Option<String>>,
    mut is_loading: Signal<bool>,
) {
    let text = input_text.read().trim().to_string();
    let image = attached_image_data.read().clone();

    if text.is_empty() && image.is_none() {
        return;
    }

    let api_key = match api_key_opt {
        Some(k) if !k.trim().is_empty() => k.to_string(),
        _ => {
            error_msg.set(Some("Silakan masukkan Google Gemini API Key Anda terlebih dahulu.".to_string()));
            return;
        }
    };

    let user_msg = AiChatMessage {
        id: generate_id(),
        role: AiMessageRole::User,
        text: if text.is_empty() { "Analisis lampiran foto ini.".to_string() } else { text.clone() },
        proposed_action: None,
        action_status: AiActionStatus::Pending,
        attachment_preview: image.clone(),
        timestamp: get_current_time_hm(),
    };

    let mut current_history = messages.read().clone();
    current_history.push(user_msg);
    messages.set(current_history.clone());
    save_ai_chat_history(&current_history);

    input_text.set(String::new());
    attached_image_data.set(None);
    attached_image_name.set(None);
    error_msg.set(None);
    is_loading.set(true);

    scroll_chat_to_bottom();

    let system_prompt = build_system_context(
        user_name,
        wallets,
        trxs,
        cats,
        budgets,
        savings,
    );

    spawn({
        let mut messages_sig = messages;
        let mut is_loading_sig = is_loading;
        let mut error_sig = error_msg;
        let current_msgs = current_history.clone();
        let prompt_text = text;
        let img_data = image;

        async move {
            let result = call_gemini_api(
                &api_key,
                &system_prompt,
                &current_msgs,
                &prompt_text,
                img_data.as_deref(),
            ).await;

            is_loading_sig.set(false);

            match result {
                Ok((reply_text, proposed_action)) => {
                    let assistant_msg = AiChatMessage {
                        id: generate_id(),
                        role: AiMessageRole::Assistant,
                        text: reply_text,
                        proposed_action,
                        action_status: AiActionStatus::Pending,
                        attachment_preview: None,
                        timestamp: get_current_time_hm(),
                    };

                    let mut updated = messages_sig.read().clone();
                    updated.push(assistant_msg);
                    messages_sig.set(updated.clone());
                    save_ai_chat_history(&updated);
                }
                Err(err) => {
                    error_sig.set(Some(err));
                }
            }
        }
    });
}

/// Helper function to build dynamic system context from actual user financial data
pub fn build_system_context(
    user_name: &str,
    wallets: &[Wallet],
    transactions: &[Transaction],
    categories: &UserCategories,
    budgets: &[CategoryBudget],
    savings_goals: &[SavingsGoal],
) -> String {
    let today = get_today_date();
    let current_time = get_current_time_hm();

    let mut wallets_info = String::new();
    for w in wallets {
        let bal = calculate_wallet_balance(w, transactions);
        wallets_info.push_str(&format!("- {} (Tipe: {}, Saldo saat ini: {})\n", w.name, w.wallet_type.as_str(), format_idr(bal)));
    }

    let expense_cats = categories.expense.join(", ");
    let income_cats = categories.income.join(", ");

    let mut budgets_info = String::new();
    if budgets.is_empty() {
        budgets_info.push_str("Belum ada anggaran bulanan yang diatur.\n");
    } else {
        for b in budgets {
            budgets_info.push_str(&format!("- Kategori {}: Batas {}\n", b.category, format_idr(b.monthly_limit)));
        }
    }

    let mut savings_info = String::new();
    if savings_goals.is_empty() {
        savings_info.push_str("Belum ada target tabungan yang dibuat.\n");
    } else {
        for s in savings_goals {
            savings_info.push_str(&format!("- Target \"{}\" (Kategori: {}): Terkumpul {} dari target {}\n", s.name, s.category, format_idr(s.current_amount), format_idr(s.target_amount)));
        }
    }

    format!(
r#"Anda adalah Asisten Keuangan Pribadi AI cerdas dan efisien untuk aplikasi CatatMoney (Monochrome Swiss FinTech Minimalist).
Nama Pengguna: {user_name}
Tanggal Sekarang: {today}
Waktu Sekarang: {current_time} WIB

KONTEKS KEUANGAN PENGGUNA SAAT INI:
[Daftar Akun / Dompet]:
{wallets_info}

[Kategori Pengeluaran Resmi]:
{expense_cats}

[Kategori Pemasukan Resmi]:
{income_cats}

[Status Anggaran]:
{budgets_info}

[Status Target Tabungan]:
{savings_info}

ATURAN PERILAKU DAN RESPON:
1. PENTING: DILARANG MENGGUNAKAN KARAKTER EMOJI APAPUN dalam seluruh teks balasan (patuhi aturan strict zero-emoji CatatMoney).
2. Jika pengguna meminta mencatat transaksi, membeli sesuatu, atau mengirim foto struk/nota, SELALU panggil fungsi `record_transaction`. Pilih dompet dan kategori yang paling cocok dari daftar resmi di atas. Jika pengguna tidak menyebutkan nama dompet, gunakan dompet pertama yang memiliki saldo mencukupi.
3. Jika pengguna mengirim foto struk/nota/invoice: lakukan ekstraksi OCR untuk menemukan nama toko/merchant, total tagihan/harga, tanggal (jika ada), dan tetapkan kategori belanja/makanan yang sesuai.
4. Jika pengguna ingin menabung/setor uang ke impian tertentu, panggil `deposit_savings`.
5. Jika pengguna ingin menarik tabungan, panggil `withdraw_savings`.
6. Jika pengguna ingin membatasi atau mengatur anggaran belanja, panggil `set_budget`.
7. Jika pengguna hanya bertanya seputar kondisi keuangan (misal: sisa anggaran, pengeluaran terbesar, rekomendasi), jawablah secara ringkas, to the point, dan berbasis angka nyata di atas tanpa memanggil function call.
8. Selalu gunakan format nominal Rupiah yang ramah dibaca manusia pada teks penjelasan Anda."#
    )
}

/// Helper function to perform async HTTP POST to Gemini REST API
async fn call_gemini_api(
    api_key: &str,
    system_prompt: &str,
    chat_history: &[AiChatMessage],
    user_prompt: &str,
    image_base64: Option<&str>,
) -> Result<(String, Option<AiProposedAction>), String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        GEMINI_MODEL,
        api_key
    );

    // Build contents array
    let mut contents = Vec::new();

    // Add up to 8 recent messages for conversation context
    let history_slice = if chat_history.len() > 8 {
        &chat_history[chat_history.len() - 8..]
    } else {
        chat_history
    };

    for msg in history_slice {
        let role_str = match msg.role {
            AiMessageRole::User => "user",
            AiMessageRole::Assistant => "model",
        };
        contents.push(serde_json::json!({
            "role": role_str,
            "parts": [{ "text": msg.text }]
        }));
    }

    // Current turn parts
    let mut current_parts = Vec::new();
    if !user_prompt.trim().is_empty() {
        current_parts.push(serde_json::json!({ "text": user_prompt }));
    }
    if let Some(img_data) = image_base64 {
        let (mime_type, base64_raw) = if let Some(idx) = img_data.find(";base64,") {
            let mime = &img_data[5..idx];
            let raw = &img_data[idx + 8..];
            (mime, raw)
        } else {
            ("image/jpeg", img_data)
        };
        current_parts.push(serde_json::json!({
            "inlineData": {
                "mimeType": mime_type,
                "data": base64_raw
            }
        }));
    }
    if current_parts.is_empty() {
        current_parts.push(serde_json::json!({ "text": "Mohon periksa data ini." }));
    }

    contents.push(serde_json::json!({
        "role": "user",
        "parts": current_parts
    }));

    // Function declarations
    let tools = serde_json::json!([
        {
            "functionDeclarations": [
                {
                    "name": "record_transaction",
                    "description": "Mencatat transaksi keuangan baru (pengeluaran, pemasukan, atau transfer antar akun). Panggil fungsi ini jika pengguna berniat mencatat transaksi atau mengirim foto struk belanja.",
                    "parameters": {
                        "type": "OBJECT",
                        "properties": {
                            "title": {
                                "type": "STRING",
                                "description": "Judul atau deskripsi singkat transaksi (misal: Makan Siang, Belanja Bulanan, Gaji)"
                            },
                            "amount": {
                                "type": "NUMBER",
                                "description": "Nominal transaksi dalam Rupiah (angka murni positif tanpa tanda titik/koma)"
                            },
                            "transaction_type": {
                                "type": "STRING",
                                "enum": ["Expense", "Income", "Transfer"],
                                "description": "Tipe transaksi: Expense (pengeluaran), Income (pemasukan), atau Transfer (antar akun)"
                            },
                            "category": {
                                "type": "STRING",
                                "description": "Nama kategori transaksi. Wajib pilih salah satu dari kategori yang tersedia."
                            },
                            "wallet": {
                                "type": "STRING",
                                "description": "Nama dompet atau akun sumber dana. Wajib pilih salah satu dari dompet pengguna yang tersedia."
                            },
                            "to_wallet": {
                                "type": "STRING",
                                "description": "Nama dompet tujuan (hanya jika tipe transaksi adalah Transfer)"
                            },
                            "date": {
                                "type": "STRING",
                                "description": "Tanggal transaksi format YYYY-MM-DD"
                            },
                            "time": {
                                "type": "STRING",
                                "description": "Jam transaksi format HH:MM"
                            },
                            "notes": {
                                "type": "STRING",
                                "description": "Catatan tambahan rincian transaksi atau isi struk"
                            }
                        },
                        "required": ["title", "amount", "transaction_type", "category", "wallet"]
                    }
                },
                {
                    "name": "deposit_savings",
                    "description": "Menyetor atau menambah saldo ke target tabungan impian pengguna.",
                    "parameters": {
                        "type": "OBJECT",
                        "properties": {
                            "goal_name": {
                                "type": "STRING",
                                "description": "Nama target tabungan tujuan yang terdaftar"
                            },
                            "amount": {
                                "type": "NUMBER",
                                "description": "Nominal setoran dalam Rupiah"
                            },
                            "notes": {
                                "type": "STRING",
                                "description": "Catatan setoran"
                            }
                        },
                        "required": ["goal_name", "amount"]
                    }
                },
                {
                    "name": "withdraw_savings",
                    "description": "Menarik dana dari target tabungan pengguna.",
                    "parameters": {
                        "type": "OBJECT",
                        "properties": {
                            "goal_name": {
                                "type": "STRING",
                                "description": "Nama target tabungan yang ingin ditarik saldonya"
                            },
                            "amount": {
                                "type": "NUMBER",
                                "description": "Nominal penarikan dalam Rupiah"
                            },
                            "notes": {
                                "type": "STRING",
                                "description": "Alasan atau catatan penarikan dana"
                            }
                        },
                        "required": ["goal_name", "amount"]
                    }
                },
                {
                    "name": "set_budget",
                    "description": "Mengatur atau memperbarui batas pagu anggaran bulanan untuk suatu kategori pengeluaran.",
                    "parameters": {
                        "type": "OBJECT",
                        "properties": {
                            "category": {
                                "type": "STRING",
                                "description": "Nama kategori pengeluaran yang diatur batas anggarannya"
                            },
                            "monthly_limit": {
                                "type": "NUMBER",
                                "description": "Batas anggaran maksimal per bulan dalam Rupiah"
                            }
                        },
                        "required": ["category", "monthly_limit"]
                    }
                }
            ]
        }
    ]);

    let request_body = serde_json::json!({
        "systemInstruction": {
            "parts": [{ "text": system_prompt }]
        },
        "contents": contents,
        "tools": tools,
        "generationConfig": {
            "temperature": 0.2,
            "maxOutputTokens": 1024
        }
    });

    let response = gloo_net::http::Request::post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Gagal memformat request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Koneksi ke Gemini gagal. Periksa koneksi internet: {}", e))?;

    if !response.ok() {
        let status = response.status();
        let err_text = response.text().await.unwrap_or_default();
        if status == 400 || status == 403 {
            return Err("API Key Gemini tidak valid atau kuota habis. Silakan periksa kembali API Key Anda.".to_string());
        } else if status == 404 {
            return Err("Layanan AI Gemini sedang tidak dapat diakses atau endpoint tidak ditemukan (404). Silakan coba beberapa saat lagi.".to_string());
        } else {
            return Err(format!("Gemini API Error ({}): {}", status, err_text));
        }
    }

    let res_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Gagal membaca respons dari Gemini: {}", e))?;

    let mut text_response = String::new();
    let mut proposed_action = None;

    if let Some(parts) = res_json["candidates"][0]["content"]["parts"].as_array() {
        for part in parts {
            if let Some(t) = part["text"].as_str() {
                if !text_response.is_empty() {
                    text_response.push('\n');
                }
                text_response.push_str(t);
            }

            if let Some(fc) = part["functionCall"].as_object() {
                let fn_name = fc.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = fc.get("args").and_then(|v| v.as_object());

                if let Some(args) = args {
                    match fn_name {
                        "record_transaction" => {
                            let title = args.get("title").and_then(|v| v.as_str()).unwrap_or("Transaksi").to_string();
                            let amount = args.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let t_type_str = args.get("transaction_type").and_then(|v| v.as_str()).unwrap_or("Expense");
                            let trx_type = match t_type_str {
                                "Income" => TransactionType::Income,
                                "Transfer" => TransactionType::Transfer,
                                _ => TransactionType::Expense,
                            };
                            let category = args.get("category").and_then(|v| v.as_str()).unwrap_or("Lainnya").to_string();
                            let wallet = args.get("wallet").and_then(|v| v.as_str()).unwrap_or("BCA").to_string();
                            let date = args.get("date").and_then(|v| v.as_str()).unwrap_or_else(|| "").to_string();
                            let date = if date.len() == 10 { date } else { get_today_date() };
                            let time = args.get("time").and_then(|v| v.as_str()).unwrap_or_else(|| "").to_string();
                            let time = if time.len() >= 4 { time } else { get_current_time_hm() };
                            let notes = args.get("notes").and_then(|v| v.as_str()).unwrap_or("").to_string();

                            proposed_action = Some(AiProposedAction::RecordTransaction {
                                title,
                                amount,
                                transaction_type: trx_type,
                                category,
                                wallet,
                                date,
                                time,
                                notes,
                                attachment: image_base64.map(|s| s.to_string()),
                            });
                        }
                        "deposit_savings" => {
                            let goal_name = args.get("goal_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let amount = args.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let notes = args.get("notes").and_then(|v| v.as_str()).unwrap_or("Setoran via Gemini AI").to_string();

                            proposed_action = Some(AiProposedAction::DepositSavings {
                                goal_name,
                                amount,
                                notes,
                            });
                        }
                        "withdraw_savings" => {
                            let goal_name = args.get("goal_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let amount = args.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let notes = args.get("notes").and_then(|v| v.as_str()).unwrap_or("Penarikan via Gemini AI").to_string();

                            proposed_action = Some(AiProposedAction::WithdrawSavings {
                                goal_name,
                                amount,
                                notes,
                            });
                        }
                        "set_budget" => {
                            let category = args.get("category").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let monthly_limit = args.get("monthly_limit").and_then(|v| v.as_f64()).unwrap_or(0.0);

                            proposed_action = Some(AiProposedAction::SetBudget {
                                category,
                                monthly_limit,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if text_response.is_empty() && proposed_action.is_some() {
        text_response = "Saya telah menyiapkan rincian transaksi berikut untuk Anda. Silakan periksa dan konfirmasi:".to_string();
    } else if text_response.is_empty() {
        text_response = "Maaf, saya tidak dapat memahami permintaan tersebut. Mohon ulangi instruksi Anda.".to_string();
    }

    Ok((text_response, proposed_action))
}
