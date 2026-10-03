use dioxus::prelude::*;
use crate::components::icons::{
    CategoryIcon, IconAlertTriangle, IconArrowDownRight, IconArrowUpRight, IconBanknote, IconCheck,
    IconCreditCard, IconDownload, IconEdit, IconLandmark, IconMoon, IconPlus, IconSliders,
    IconSmartphone, IconSparkles, IconSun, IconTrash, IconUpload, IconWallet, IconX, WalletIcon,
};
#[allow(unused_imports)]
use crate::model::{
    create_backup, format_idr, generate_id, get_today_date, parse_input_idr,
    trigger_json_download, CatatMoneyBackup, ThemeMode, Transaction, TransactionType,
    UserCategories, UserProfile, Wallet, WalletType,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ManageTab {
    Wallets,
    ExpenseCategories,
    IncomeCategories,
    Theme,
    BackupRestore,
}

#[component]
pub fn ManagementView(
    profile: Option<UserProfile>,
    transactions: Vec<Transaction>,
    wallets: Vec<Wallet>,
    categories: UserCategories,
    current_theme: ThemeMode,
    on_change_theme: EventHandler<ThemeMode>,
    on_update_wallets: EventHandler<Vec<Wallet>>,
    on_update_categories: EventHandler<UserCategories>,
    on_request_delete_wallet: EventHandler<String>,
    on_request_delete_category: EventHandler<(TransactionType, String)>,
    on_restore_data: EventHandler<(Option<UserProfile>, Vec<Transaction>, Vec<Wallet>, UserCategories)>,
    on_request_reset_all_data: EventHandler<()>,
) -> Element {
    let mut active_subtab = use_signal(|| ManageTab::Wallets);

    // State form Sumber Dana
    let mut is_adding_wallet = use_signal(|| false);
    let mut editing_wallet_id = use_signal(|| None::<String>);
    let mut wallet_name_input = use_signal(String::new);
    let mut wallet_type_input = use_signal(|| WalletType::Bank);
    let mut wallet_balance_input = use_signal(|| 0.0);
    let mut wallet_balance_display = use_signal(|| "Rp 0".to_string());
    let mut wallet_error = use_signal(|| None::<String>);

    // State form Kategori
    let mut is_adding_cat = use_signal(|| false);
    let mut editing_cat_orig = use_signal(|| None::<String>);
    let mut cat_name_input = use_signal(String::new);
    let mut cat_error = use_signal(|| None::<String>);

    // State pesan Backup & Restore
    let mut restore_status_msg = use_signal(|| None::<String>);
    let mut restore_error_msg = use_signal(|| None::<String>);

    rsx! {
        div { class: "management-container surface-panel",
            // Header Manajemen
            div { class: "panel-header flex items-center justify-between pb-4 border-b border-[var(--border-subtle)]",
                div {
                    h2 { class: "panel-title flex items-center gap-2",
                        IconSliders { size: "18" }
                        "Pusat Manajemen Data"
                    }
                    p { class: "panel-subtitle text-xs text-muted mt-0.5",
                        "Kelola akun sumber dana, kategori transaksi, serta cadangan arsip keuangan Anda"
                    }
                }
            }

            // Sub-Tab Switcher
            div { class: "manage-subtabs-bar mt-4 mb-6",
                button {
                    r#type: "button",
                    class: if *active_subtab.read() == ManageTab::Wallets { "manage-tab-btn active" } else { "manage-tab-btn" },
                    onclick: move |_| {
                        active_subtab.set(ManageTab::Wallets);
                        is_adding_wallet.set(false);
                        editing_wallet_id.set(None);
                        wallet_error.set(None);
                    },
                    IconWallet { size: "15" }
                    span { "Sumber Dana ({wallets.len()})" }
                }
                button {
                    r#type: "button",
                    class: if *active_subtab.read() == ManageTab::ExpenseCategories { "manage-tab-btn active" } else { "manage-tab-btn" },
                    onclick: move |_| {
                        active_subtab.set(ManageTab::ExpenseCategories);
                        is_adding_cat.set(false);
                        editing_cat_orig.set(None);
                        cat_error.set(None);
                    },
                    IconArrowDownRight { size: "15" }
                    span { class: "tab-label-full", "Kategori Pengeluaran ({categories.expense.len()})" }
                    span { class: "tab-label-short", "Pengeluaran ({categories.expense.len()})" }
                }
                button {
                    r#type: "button",
                    class: if *active_subtab.read() == ManageTab::IncomeCategories { "manage-tab-btn active" } else { "manage-tab-btn" },
                    onclick: move |_| {
                        active_subtab.set(ManageTab::IncomeCategories);
                        is_adding_cat.set(false);
                        editing_cat_orig.set(None);
                        cat_error.set(None);
                    },
                    IconArrowUpRight { size: "15" }
                    span { class: "tab-label-full", "Kategori Pemasukan ({categories.income.len()})" }
                    span { class: "tab-label-short", "Pemasukan ({categories.income.len()})" }
                }
                button {
                    r#type: "button",
                    class: if *active_subtab.read() == ManageTab::Theme { "manage-tab-btn active" } else { "manage-tab-btn" },
                    onclick: move |_| {
                        active_subtab.set(ManageTab::Theme);
                        is_adding_wallet.set(false);
                        editing_wallet_id.set(None);
                        is_adding_cat.set(false);
                    },
                    if current_theme == ThemeMode::Dark {
                        IconMoon { size: "15" }
                    } else {
                        IconSun { size: "15" }
                    }
                    span { class: "tab-label-full", "Tema & Tampilan" }
                    span { class: "tab-label-short", "Tema" }
                }
                button {
                    r#type: "button",
                    class: if *active_subtab.read() == ManageTab::BackupRestore { "manage-tab-btn active" } else { "manage-tab-btn" },
                    onclick: move |_| {
                        active_subtab.set(ManageTab::BackupRestore);
                        is_adding_wallet.set(false);
                        editing_wallet_id.set(None);
                        is_adding_cat.set(false);
                        restore_error_msg.set(None);
                    },
                    IconDownload { size: "15" }
                    span { class: "tab-label-full", "Cadangan Data (Backup)" }
                    span { class: "tab-label-short", "Cadangan Data" }
                }
            }

            // Konten Sesuai Sub-Tab
            match *active_subtab.read() {
                // 1. MANAJEMEN SUMBER DANA
                ManageTab::Wallets => rsx! {
                    div { class: "manage-section",
                        div { class: "manage-section-header mb-4",
                            div {
                                h3 { class: "text-sm font-bold", "Daftar Rekening & Pos Keuangan" }
                                p { class: "text-xs text-muted", "Sumber dana digunakan sebagai asal/tujuan transaksi" }
                            }
                            if !*is_adding_wallet.read() && editing_wallet_id.read().is_none() {
                                button {
                                    r#type: "button",
                                    class: "btn-primary text-xs py-1.5 px-3 flex items-center gap-1.5 shrink-0 whitespace-nowrap",
                                    onclick: move |_| {
                                        wallet_name_input.set(String::new());
                                        wallet_type_input.set(WalletType::Bank);
                                        wallet_balance_input.set(0.0);
                                        wallet_balance_display.set("Rp 0".to_string());
                                        editing_wallet_id.set(None);
                                        is_adding_wallet.set(true);
                                    },
                                    IconPlus { size: "14" }
                                    "Tambah Sumber Dana"
                                }
                            }
                        }

                        if let Some(err) = wallet_error.read().as_ref() {
                            div { class: "form-error mb-4", "{err}" }
                        }

                        // Form Tambah / Edit Sumber Dana (TANPA NOMINAL UANG)
                        if *is_adding_wallet.read() || editing_wallet_id.read().is_some() {
                            div { class: "manage-form-card mb-5 p-4 rounded-xl border border-[var(--border-color)] bg-[var(--bg-surface-subtle)]",
                                div { class: "flex items-center justify-between mb-3 pb-2 border-b border-[var(--border-subtle)]",
                                    h4 { class: "text-xs font-bold uppercase tracking-wider text-[var(--text-secondary)]",
                                        if editing_wallet_id.read().is_some() { "Edit Sumber Dana" } else { "Pendaftaran Sumber Dana Baru" }
                                    }
                                    button {
                                        r#type: "button",
                                        class: "btn-icon-close",
                                        onclick: move |_| {
                                            is_adding_wallet.set(false);
                                            editing_wallet_id.set(None);
                                        },
                                        IconX { size: "15" }
                                    }
                                }

                                form {
                                    onsubmit: {
                                        let wallets_list = wallets.clone();
                                        move |e: FormEvent| {
                                            e.prevent_default();
                                            let name = wallet_name_input.read().trim().to_string();
                                            if name.is_empty() {
                                                wallet_error.set(Some("Nama sumber dana tidak boleh kosong.".to_string()));
                                                return;
                                            }

                                            let mut updated = wallets_list.clone();
                                            if let Some(ref edit_id) = *editing_wallet_id.read() {
                                                if let Some(w) = updated.iter_mut().find(|w| &w.id == edit_id) {
                                                    w.name = name;
                                                    w.wallet_type = *wallet_type_input.read();
                                                    w.initial_balance = *wallet_balance_input.read();
                                                }
                                            } else {
                                                let new_w = Wallet {
                                                    id: generate_id(),
                                                    name,
                                                    wallet_type: *wallet_type_input.read(),
                                                    initial_balance: *wallet_balance_input.read(),
                                                };
                                                updated.push(new_w);
                                            }

                                            on_update_wallets.call(updated);
                                            is_adding_wallet.set(false);
                                            editing_wallet_id.set(None);
                                            wallet_name_input.set(String::new());
                                            wallet_balance_input.set(0.0);
                                            wallet_balance_display.set("Rp 0".to_string());
                                            wallet_error.set(None);
                                        }
                                    },

                                    div { class: "field-group mb-3",
                                        label { class: "field-label", "Nama Sumber Dana / Akun" }
                                        input {
                                            r#type: "text",
                                            class: "field-input",
                                            placeholder: "Misal: Bank Mandiri, DANA, Brankas Tunai",
                                            value: "{wallet_name_input}",
                                            oninput: move |e| wallet_name_input.set(e.value()),
                                        }
                                    }

                                    div { class: "field-group mb-4",
                                        label { class: "field-label", "Tipe Akun" }
                                        div { class: "wallet-type-grid",
                                            button {
                                                r#type: "button",
                                                class: if *wallet_type_input.read() == WalletType::Bank { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                                onclick: move |_| wallet_type_input.set(WalletType::Bank),
                                                IconLandmark { size: "15" }
                                                span { "Rekening Bank" }
                                            }
                                            button {
                                                r#type: "button",
                                                class: if *wallet_type_input.read() == WalletType::EWallet { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                                onclick: move |_| wallet_type_input.set(WalletType::EWallet),
                                                IconSmartphone { size: "15" }
                                                span { "E-Wallet" }
                                            }
                                            button {
                                                r#type: "button",
                                                class: if *wallet_type_input.read() == WalletType::Cash { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                                onclick: move |_| wallet_type_input.set(WalletType::Cash),
                                                IconBanknote { size: "15" }
                                                span { "Uang Tunai" }
                                            }
                                            button {
                                                r#type: "button",
                                                class: if *wallet_type_input.read() == WalletType::CreditCard { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                                onclick: move |_| wallet_type_input.set(WalletType::CreditCard),
                                                IconCreditCard { size: "15" }
                                                span { "Kartu Kredit" }
                                            }
                                            button {
                                                r#type: "button",
                                                class: if *wallet_type_input.read() == WalletType::Other { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                                onclick: move |_| wallet_type_input.set(WalletType::Other),
                                                IconWallet { size: "15" }
                                                span { "Lainnya" }
                                            }
                                        }
                                    }

                                    div { class: "field-group mb-4",
                                        label { class: "field-label", "Saldo Awal Saat Ini" }
                                        input {
                                            r#type: "text",
                                            class: "field-input text-sm tabular-numbers",
                                            placeholder: "Rp 0",
                                            value: "{wallet_balance_display}",
                                            oninput: move |e| {
                                                let (val, formatted) = parse_input_idr(&e.value());
                                                wallet_balance_input.set(val);
                                                if val == 0.0 {
                                                    wallet_balance_display.set("Rp 0".to_string());
                                                } else {
                                                    wallet_balance_display.set(format!("Rp {}", formatted));
                                                }
                                            },
                                        }
                                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                                            "Saldo nyata akun ini saat pertama kali didaftarkan. Kosongkan atau biarkan Rp 0 jika belum ada saldo."
                                        }
                                    }

                                    div { class: "flex items-center justify-end gap-2 mt-4 pt-3 border-t border-[var(--border-subtle)]",
                                        button {
                                            r#type: "button",
                                            class: "btn-secondary text-xs py-1.5 px-3",
                                            onclick: move |_| {
                                                is_adding_wallet.set(false);
                                                editing_wallet_id.set(None);
                                            },
                                            "Batal"
                                        }
                                        button {
                                            r#type: "submit",
                                            class: "btn-primary text-xs py-1.5 px-4 flex items-center gap-1.5",
                                            IconCheck { size: "14" }
                                            "Simpan Sumber Dana"
                                        }
                                    }
                                }
                            }
                        }

                        // List Kartu Sumber Dana
                        div { class: "manage-item-grid",
                            for w in wallets.iter() {
                                {
                                    let id = w.id.clone();
                                    let name = w.name.clone();
                                    let w_type = w.wallet_type;
                                    let w_bal = w.initial_balance;
                                    let is_editing = editing_wallet_id.read().as_ref() == Some(&id);

                                    rsx! {
                                        div {
                                            class: if is_editing { "manage-item-card active-editing" } else { "manage-item-card" },
                                            key: "{w.id}",
                                            div { class: "manage-item-main",
                                                div { class: "wallet-icon-box",
                                                    WalletIcon { wallet_type: w.wallet_type, size: "16" }
                                                }
                                                div {
                                                    div { class: "manage-item-name", "{w.name}" }
                                                    span { class: "wallet-type-text mt-0.5 inline-block",
                                                        "{w.wallet_type.as_str()}"
                                                    }
                                                }
                                            }
                                            div { class: "manage-item-actions",
                                                button {
                                                    r#type: "button",
                                                    class: "btn-icon-subtle",
                                                    title: "Edit Sumber Dana",
                                                    onclick: move |_| {
                                                        wallet_name_input.set(name.clone());
                                                        wallet_type_input.set(w_type);
                                                        wallet_balance_input.set(w_bal);
                                                        if w_bal == 0.0 {
                                                            wallet_balance_display.set("Rp 0".to_string());
                                                        } else {
                                                            wallet_balance_display.set(format_idr(w_bal));
                                                        }
                                                        editing_wallet_id.set(Some(id.clone()));
                                                        is_adding_wallet.set(false);
                                                        wallet_error.set(None);
                                                    },
                                                    IconEdit { size: "14" }
                                                }
                                                button {
                                                    r#type: "button",
                                                    class: "btn-icon-danger",
                                                    title: "Hapus Sumber Dana",
                                                    onclick: {
                                                        let del_name = w.name.clone();
                                                        let wallets_list = wallets.clone();
                                                        move |_| {
                                                            if wallets_list.len() <= 1 {
                                                                wallet_error.set(Some("Minimal harus ada 1 sumber dana aktif.".to_string()));
                                                                return;
                                                            }
                                                            on_request_delete_wallet.call(del_name.clone());
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
                },

                // 2. MANAJEMEN KATEGORI PENGELUARAN / PEMASUKAN
                ManageTab::ExpenseCategories | ManageTab::IncomeCategories => {
                    let is_expense = *active_subtab.read() == ManageTab::ExpenseCategories;
                    let label_kind = if is_expense { "Pengeluaran" } else { "Pemasukan" };
                    let current_list = if is_expense { categories.expense.clone() } else { categories.income.clone() };

                    rsx! {
                        div { class: "manage-section",
                            div { class: "manage-section-header mb-4",
                                div {
                                    h3 { class: "text-sm font-bold", "Kategori {label_kind}" }
                                    p { class: "text-xs text-muted", "Kategori yang digunakan untuk klasifikasi transaksi {label_kind}" }
                                }
                                if !*is_adding_cat.read() && editing_cat_orig.read().is_none() {
                                    button {
                                        r#type: "button",
                                        class: "btn-primary text-xs py-1.5 px-3 flex items-center gap-1.5 shrink-0 whitespace-nowrap",
                                        onclick: move |_| {
                                            cat_name_input.set(String::new());
                                            editing_cat_orig.set(None);
                                            is_adding_cat.set(true);
                                            cat_error.set(None);
                                        },
                                        IconPlus { size: "14" }
                                        "Tambah Kategori"
                                    }
                                }
                            }

                            if let Some(err) = cat_error.read().as_ref() {
                                div { class: "form-error mb-4", "{err}" }
                            }

                            // Form Tambah / Edit Kategori
                            if *is_adding_cat.read() || editing_cat_orig.read().is_some() {
                                div { class: "manage-form-card mb-5 p-4 rounded-xl border border-[var(--border-color)] bg-[var(--bg-surface-subtle)]",
                                    div { class: "flex items-center justify-between mb-3 pb-2 border-b border-[var(--border-subtle)]",
                                        h4 { class: "text-xs font-bold uppercase tracking-wider text-[var(--text-secondary)]",
                                            if editing_cat_orig.read().is_some() { "Edit Kategori {label_kind}" } else { "Tambah Kategori {label_kind} Baru" }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "btn-icon-close",
                                            onclick: move |_| {
                                                is_adding_cat.set(false);
                                                editing_cat_orig.set(None);
                                            },
                                            IconX { size: "15" }
                                        }
                                    }

                                    form {
                                        onsubmit: {
                                            let cats = categories.clone();
                                            move |e: FormEvent| {
                                                e.prevent_default();
                                                let name = cat_name_input.read().trim().to_string();
                                                if name.is_empty() {
                                                    cat_error.set(Some("Nama kategori tidak boleh kosong.".to_string()));
                                                    return;
                                                }

                                                let mut updated = cats.clone();
                                                let target_list = if is_expense {
                                                    &mut updated.expense
                                                } else {
                                                    &mut updated.income
                                                };

                                                if let Some(ref orig) = *editing_cat_orig.read() {
                                                    if let Some(pos) = target_list.iter().position(|c| c == orig) {
                                                        target_list[pos] = name;
                                                    }
                                                } else {
                                                    if target_list.contains(&name) {
                                                        cat_error.set(Some("Kategori dengan nama tersebut sudah ada.".to_string()));
                                                        return;
                                                    }
                                                    target_list.push(name);
                                                }

                                                on_update_categories.call(updated);
                                                is_adding_cat.set(false);
                                                editing_cat_orig.set(None);
                                                cat_name_input.set(String::new());
                                                cat_error.set(None);
                                            }
                                        },

                                        div { class: "field-group mb-4",
                                            label { class: "field-label", "Nama Kategori" }
                                            input {
                                                r#type: "text",
                                                class: "field-input",
                                                placeholder: if is_expense { "Misal: Langganan SaaS, Pajak, Hobi" } else { "Misal: Dividen, Saham, Hibah" },
                                                value: "{cat_name_input}",
                                                oninput: move |e| cat_name_input.set(e.value()),
                                            }
                                        }

                                        div { class: "flex items-center justify-end gap-2 pt-3 border-t border-[var(--border-subtle)]",
                                            button {
                                                r#type: "button",
                                                class: "btn-secondary text-xs py-1.5 px-3",
                                                onclick: move |_| {
                                                    is_adding_cat.set(false);
                                                    editing_cat_orig.set(None);
                                                },
                                                "Batal"
                                            }
                                            button {
                                                r#type: "submit",
                                                class: "btn-primary text-xs py-1.5 px-4 flex items-center gap-1.5",
                                                IconCheck { size: "14" }
                                                "Simpan Kategori"
                                            }
                                        }
                                    }
                                }
                            }

                            // List Kategori
                            div { class: "manage-item-grid",
                                for cat in current_list {
                                    {
                                        let c_name = cat.clone();
                                        let is_editing = editing_cat_orig.read().as_ref() == Some(&c_name);
                                        let box_class = if is_expense { "category-icon-box expense" } else { "category-icon-box income" };

                                        rsx! {
                                            div {
                                                class: if is_editing { "manage-item-card active-editing" } else { "manage-item-card" },
                                                key: "{cat}",
                                                div { class: "manage-item-main",
                                                    div { class: "{box_class}",
                                                        CategoryIcon { category: cat.clone() }
                                                    }
                                                    div {
                                                        div { class: "manage-item-name", "{cat}" }
                                                        span { class: "text-xs text-muted", "Kategori {label_kind}" }
                                                    }
                                                }
                                                div { class: "manage-item-actions",
                                                    button {
                                                        r#type: "button",
                                                        class: "btn-icon-subtle",
                                                        title: "Edit Kategori",
                                                        onclick: {
                                                            let cn = c_name.clone();
                                                            move |_| {
                                                                cat_name_input.set(cn.clone());
                                                                editing_cat_orig.set(Some(cn.clone()));
                                                                is_adding_cat.set(false);
                                                                cat_error.set(None);
                                                            }
                                                        },
                                                        IconEdit { size: "14" }
                                                    }
                                                    button {
                                                        r#type: "button",
                                                        class: "btn-icon-danger",
                                                        title: "Hapus Kategori",
                                                        onclick: {
                                                            let del_name = c_name.clone();
                                                            let cats = categories.clone();
                                                            move |_| {
                                                                let is_exp = *active_subtab.read() == ManageTab::ExpenseCategories;
                                                                let target_list_len = if is_exp { cats.expense.len() } else { cats.income.len() };
                                                                if target_list_len <= 1 {
                                                                    cat_error.set(Some("Minimal harus ada 1 kategori.".to_string()));
                                                                    return;
                                                                }
                                                                on_request_delete_category.call((
                                                                    if is_exp { TransactionType::Expense } else { TransactionType::Income },
                                                                    del_name.clone(),
                                                                ));
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
                },

                // 3. TEMA & TAMPILAN ANTARMUKA
                ManageTab::Theme => rsx! {
                    div { class: "manage-section",
                        div { class: "manage-section-header mb-5",
                            div {
                                h3 { class: "text-sm font-bold text-[var(--text-primary)]", "Tema & Tampilan Antarmuka" }
                                p { class: "text-xs text-muted mt-0.5",
                                    "Pilih gaya visual CatatMoney sesuai preferensi dan kenyamanan membaca data Anda."
                                }
                            }
                        }

                        div { class: "theme-picker-grid grid grid-cols-1 sm:grid-cols-2 gap-4",
                            // Kartu Obsidian Dark
                            div {
                                class: if current_theme == ThemeMode::Dark { "theme-card active cursor-pointer" } else { "theme-card cursor-pointer" },
                                onclick: move |_| on_change_theme.call(ThemeMode::Dark),
                                div { class: "theme-card-header flex items-center justify-between mb-3",
                                    div { class: "flex items-center gap-2",
                                        div { class: "w-7 h-7 rounded-lg bg-[#18181c] border border-[#27272a] flex items-center justify-center text-[#fafafa]",
                                            IconMoon { size: "14" }
                                        }
                                        span { class: "text-sm font-bold text-[var(--text-primary)]", "Obsidian Dark" }
                                    }
                                    if current_theme == ThemeMode::Dark {
                                        span { class: "px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider bg-[var(--text-primary)] text-[var(--bg-app)]", "Aktif" }
                                    }
                                }
                                p { class: "text-xs text-[var(--text-secondary)] leading-relaxed mb-4",
                                    "Gaya gelap monokromatik Swiss FinTech berfokus pada ketajaman data dan kenyamanan mata di ruangan redup."
                                }
                                div { class: "theme-swatch-bar flex items-center gap-1.5 p-2 rounded-lg bg-[#09090b] border border-[#27272a]",
                                    div { class: "w-4 h-4 rounded bg-[#09090b] border border-[#3f3f46]", title: "Latar Belakang" }
                                    div { class: "w-4 h-4 rounded bg-[#18181c] border border-[#3f3f46]", title: "Permukaan Panel" }
                                    div { class: "w-4 h-4 rounded bg-[#27272a]", title: "Border Tipis" }
                                    div { class: "w-4 h-4 rounded bg-[#fafafa]", title: "Teks Utama" }
                                    span { class: "text-[11px] text-[#71717a] ml-auto font-mono", "#09090B" }
                                }
                            }

                            // Kartu Clean Light
                            div {
                                class: if current_theme == ThemeMode::Light { "theme-card active cursor-pointer" } else { "theme-card cursor-pointer" },
                                onclick: move |_| on_change_theme.call(ThemeMode::Light),
                                div { class: "theme-card-header flex items-center justify-between mb-3",
                                    div { class: "flex items-center gap-2",
                                        div { class: "w-7 h-7 rounded-lg bg-[#f4f4f7] border border-[#e4e4e9] flex items-center justify-center text-[#09090b]",
                                            IconSun { size: "14" }
                                        }
                                        span { class: "text-sm font-bold text-[var(--text-primary)]", "Clean Light" }
                                    }
                                    if current_theme == ThemeMode::Light {
                                        span { class: "px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider bg-[var(--text-primary)] text-[var(--bg-app)]", "Aktif" }
                                    }
                                }
                                p { class: "text-xs text-[var(--text-secondary)] leading-relaxed mb-4",
                                    "Gaya terang bersih dengan kontras tipografi tajam, pencahayaan alami, dan kesan jernih profesional."
                                }
                                div { class: "theme-swatch-bar flex items-center gap-1.5 p-2 rounded-lg bg-[#fcfcfd] border border-[#e4e4e9]",
                                    div { class: "w-4 h-4 rounded bg-[#fcfcfd] border border-[#cbd5e1]", title: "Latar Belakang" }
                                    div { class: "w-4 h-4 rounded bg-[#ffffff] border border-[#cbd5e1]", title: "Permukaan Panel" }
                                    div { class: "w-4 h-4 rounded bg-[#e4e4e9]", title: "Border Tipis" }
                                    div { class: "w-4 h-4 rounded bg-[#09090b]", title: "Teks Utama" }
                                    span { class: "text-[11px] text-[#64748b] ml-auto font-mono", "#FCFCFD" }
                                }
                            }
                        }

                        div { class: "mt-4 p-3.5 rounded-xl border border-dashed border-[var(--border-subtle)] bg-[var(--bg-surface-subtle)] flex items-center gap-3 text-xs text-[var(--text-muted)]",
                            IconSparkles { size: "16" }
                            span { "Dukungan tema tambahan sedang dipersiapkan untuk pembaruan berikutnya." }
                        }
                    }
                },

                // 4. CADANGAN & PEMULIHAN DATA (BACKUP & RESTORE)
                ManageTab::BackupRestore => {
                    let total_trx = transactions.len();
                    let total_wallets = wallets.len();
                    let total_cats = categories.expense.len() + categories.income.len();
                    let trxs_clone = transactions.clone();
                    let wallets_clone = wallets.clone();
                    let cats_clone = categories.clone();
                    let profile_clone = profile.clone();

                    rsx! {
                        div { class: "manage-section",
                            div { class: "mb-6",
                                h3 { class: "text-sm font-bold text-[var(--text-primary)]", "Cadangan & Pemulihan Data (Backup & Restore)" }
                                p { class: "text-xs text-[var(--text-muted)] mt-0.5",
                                    "Simpan arsip seluruh data keuangan Anda ke berkas JSON lokal atau pulihkan data dari berkas cadangan kapan saja"
                                }
                            }

                            if let Some(msg) = restore_status_msg.read().as_ref() {
                                div { class: "p-3.5 mb-5 rounded-xl border border-[var(--positive-border)] bg-[var(--positive-bg)] text-[var(--positive)] flex items-center gap-2.5 text-xs font-medium",
                                    IconCheck { size: "16" }
                                    span { "{msg}" }
                                }
                            }

                            if let Some(err) = restore_error_msg.read().as_ref() {
                                div { class: "p-3.5 mb-5 rounded-xl border border-[var(--negative-border)] bg-[var(--negative-bg)] text-[var(--negative)] flex items-center gap-2.5 text-xs font-medium",
                                    IconAlertTriangle { size: "16" }
                                    span { "{err}" }
                                }
                            }

                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                // Card 1: Ekspor Cadangan Data
                                div { class: "p-5 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-col justify-between",
                                    div {
                                        div { class: "w-9 h-9 rounded-lg bg-[var(--bg-hover)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)] mb-3",
                                            IconDownload { size: "18" }
                                        }
                                        h4 { class: "text-sm font-bold text-[var(--text-primary)]", "Ekspor Cadangan (Backup JSON)" }
                                        p { class: "text-xs text-[var(--text-secondary)] mt-1.5 leading-relaxed",
                                            "Unduh seluruh data transaksi ({total_trx}), pos sumber dana ({total_wallets}), dan kategori kustom ({total_cats}) ke dalam berkas arsip .json di perangkat lokal Anda."
                                        }
                                    }
                                    div { class: "mt-6 pt-4 border-t border-[var(--border-subtle)] flex items-center justify-between",
                                        div { class: "text-xs text-[var(--text-muted)] tabular-numbers",
                                            "{total_trx} Transaksi Tersimpan"
                                        }
                                        button {
                                            r#type: "button",
                                            class: "btn-primary text-xs py-2 px-4 flex items-center gap-2",
                                            onclick: move |_| {
                                                let backup = create_backup(profile_clone.as_ref(), &trxs_clone, &wallets_clone, &cats_clone);
                                                if let Ok(json_str) = serde_json::to_string_pretty(&backup) {
                                                    let filename = format!("catatmoney_backup_{}.json", get_today_date());
                                                    trigger_json_download(&filename, &json_str);
                                                    restore_status_msg.set(Some("Berkas cadangan JSON berhasil dibuat dan diunduh.".to_string()));
                                                    restore_error_msg.set(None);
                                                }
                                            },
                                            IconDownload { size: "14" }
                                            "Unduh Backup (.json)"
                                        }
                                    }
                                }

                                // Card 2: Pulihkan Data Cadangan
                                div { class: "p-5 rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-col justify-between",
                                    div {
                                        div { class: "w-9 h-9 rounded-lg bg-[var(--bg-hover)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-primary)] mb-3",
                                            IconUpload { size: "18" }
                                        }
                                        h4 { class: "text-sm font-bold text-[var(--text-primary)]", "Pulihkan Data (Restore JSON)" }
                                        p { class: "text-xs text-[var(--text-secondary)] mt-1.5 leading-relaxed",
                                            "Pilih berkas cadangan JSON CatatMoney yang pernah diekspor untuk memulihkan seluruh data dan pengaturan secara instan."
                                        }
                                    }
                                    div { class: "mt-6 pt-4 border-t border-[var(--border-subtle)]",
                                        label {
                                            r#for: "restore-file-input",
                                            class: "btn-secondary text-xs py-2 px-4 flex items-center justify-center gap-2 cursor-pointer w-full text-center hover:border-[var(--border-hover)]",
                                            IconUpload { size: "14" }
                                            "Pilih Berkas Cadangan (.json)"
                                        }
                                        input {
                                            r#type: "file",
                                            id: "restore-file-input",
                                            accept: ".json,application/json",
                                            class: "hidden",
                                            onchange: move |_e| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    use wasm_bindgen::JsCast;
                                                    if let Some(window) = web_sys::window() {
                                                        if let Some(doc) = window.document() {
                                                            if let Some(el) = doc.get_element_by_id("restore-file-input") {
                                                                if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                                                                    if let Some(files) = input.files() {
                                                                        if let Some(file) = files.get(0) {
                                                                            if let Ok(reader) = web_sys::FileReader::new() {
                                                                                let reader_clone = reader.clone();
                                                                                let mut status_signal = restore_status_msg;
                                                                                let mut error_signal = restore_error_msg;
                                                                                let restore_cb = on_restore_data;

                                                                                let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                                                                                    if let Ok(val) = reader_clone.result() {
                                                                                        if let Some(json_text) = val.as_string() {
                                                                                            match serde_json::from_str::<CatatMoneyBackup>(&json_text) {
                                                                                                Ok(backup) => {
                                                                                                    let t_len = backup.transactions.len();
                                                                                                    let w_len = backup.wallets.len();
                                                                                                    status_signal.set(Some(format!("Berhasil memulihkan {} transaksi dan {} sumber dana!", t_len, w_len)));
                                                                                                    error_signal.set(None);
                                                                                                    restore_cb.call((backup.profile, backup.transactions, backup.wallets, backup.categories));
                                                                                                }
                                                                                                Err(_) => {
                                                                                                    error_signal.set(Some("Format berkas JSON tidak sesuai struktur data cadangan CatatMoney.".to_string()));
                                                                                                    status_signal.set(None);
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }) as Box<dyn FnMut(_)>);
                                                                                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                                                                onload.forget();
                                                                                let _ = reader.read_as_text(&file);
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

                                // Card 3: Zona Bahaya - Reset Aplikasi (Hapus Semua Data)
                                div { class: "p-5 rounded-xl border border-red-500/30 bg-red-950/10 dark:border-red-900/40 dark:bg-red-950/20 flex flex-col justify-between md:col-span-2",
                                    div { class: "flex flex-col sm:flex-row sm:items-center justify-between gap-4",
                                        div { class: "flex items-start gap-3.5",
                                            div { class: "w-9 h-9 rounded-lg bg-red-500/10 border border-red-500/20 flex items-center justify-center text-red-500 dark:text-red-400 shrink-0 mt-0.5",
                                                IconTrash { size: "18" }
                                            }
                                            div {
                                                h4 { class: "text-sm font-bold text-red-500 dark:text-red-400", "Zona Bahaya: Reset Aplikasi (Hapus Semua Data)" }
                                                p { class: "text-xs text-[var(--text-secondary)] mt-1 max-w-xl leading-relaxed",
                                                    "Hapus secara permanen seluruh catatan transaksi, pos sumber dana, kategori kustom, dan profil pengguna dari peramban ini. Aplikasi akan kembali bersih ke panduan setup awal (Getting Started)."
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "btn-danger-outline text-xs py-2 px-4 shrink-0 flex items-center justify-center gap-2",
                                            onclick: move |_| on_request_reset_all_data.call(()),
                                            IconAlertTriangle { size: "14" }
                                            "Reset Semua Data"
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
