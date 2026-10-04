use dioxus::prelude::*;

mod components;
mod model;

use components::analytics::AnalyticsView;
use components::bottom_nav::{BottomNavBar, NavTab};
use components::budget::BudgetView;
use components::calendar::FinancialCalendar;
use components::confirm_modal::ConfirmModal;
use components::dashboard::MainDashboard;
use components::detail_modal::DetailModal;
use components::form_modal::TransactionModal;
use components::history::TransactionHistory;
use components::ai_copilot::AiCopilotView;
use components::icons::{IconUser, IconWallet};
use components::management::ManagementView;
use components::onboarding::OnboardingWizard;
use components::profile_modal::ProfileModal;
use components::savings::SavingsView;
use components::wallet_modal::WalletModal;
use model::{
    format_idr, get_default_categories, load_budgets, load_categories,
    load_gemini_api_key, load_profile, load_savings_goals, load_savings_logs, load_theme,
    load_transactions, load_wallets, reset_all_data, save_budgets, save_categories,
    save_gemini_api_key, save_profile, save_savings_goals, save_savings_logs, save_theme,
    save_transactions, save_wallets, CategoryBudget, SavingsGoal, SavingsLogEntry,
    ThemeMode, Transaction, TransactionType, UserCategories, UserProfile, Wallet,
};

const APP_STYLE: &str = include_str!("../assets/style.css");
const APP_TAILWIND: &str = include_str!("../assets/tailwind.css");

#[derive(Clone, Debug, PartialEq)]
pub enum PendingDelete {
    Transaction(Transaction),
    Wallet(String),
    Category { is_expense: bool, name: String },
    AllData,
}

fn main() {
    launch(App);
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    // State preferensi tema (Light / Dark Mode Monokromatik)
    let mut theme = use_signal(load_theme);

    // Sinkronisasi kelas 'dark' / 'light' / 'tokyo-night' pada elemen root HTML browser secara reaktif
    use_effect(move || {
        let _current_mode = *theme.read();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(html) = doc.document_element() {
                        let class_list = html.class_list();
                        let _ = class_list.remove_1("dark");
                        let _ = class_list.remove_1("light");
                        let _ = class_list.remove_1("tokyo-night");
                        match _current_mode {
                            ThemeMode::Dark => {
                                let _ = class_list.add_1("dark");
                            }
                            ThemeMode::Light => {
                                let _ = class_list.add_1("light");
                            }
                            ThemeMode::TokyoNight => {
                                let _ = class_list.add_1("dark");
                                let _ = class_list.add_1("tokyo-night");
                            }
                        }
                    }
                }
            }
        }
    });

    // Hilangkan loading screen setelah aplikasi Dioxus terpasang dan styles terverifikasi
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = js_sys::eval("if (window.__dismissCatatMoneyLoader) { window.__dismissCatatMoneyLoader(); }");
        }
    });

    // State profil pengguna (disinkronkan dengan LocalStorage)
    let mut profile = use_signal(load_profile);

    // State transaksi (disinkronkan dengan LocalStorage)
    let mut transactions = use_signal(load_transactions);

    // State sumber dana / dompet (disinkronkan dengan LocalStorage)
    let mut wallets = use_signal(load_wallets);

    // State kategori (disinkronkan dengan LocalStorage)
    let mut categories = use_signal(load_categories);

    // State anggaran bulanan (disinkronkan dengan LocalStorage)
    let mut budgets = use_signal(load_budgets);

    // State target tabungan & log kontribusi (disinkronkan dengan LocalStorage)
    let mut savings_goals = use_signal(load_savings_goals);
    let mut savings_logs = use_signal(load_savings_logs);

    // State Google Gemini API Key (disinkronkan dengan LocalStorage)
    let mut gemini_api_key = use_signal(load_gemini_api_key);

    // State navigasi tab aktif (Dashboard, Kalender, Riwayat, Kelola)
    let mut active_tab = use_signal(|| NavTab::Dashboard);


    // State modal input transaksi (+)
    let mut is_modal_open = use_signal(|| false);

    // State modal tambah sumber dana
    let mut is_wallet_modal_open = use_signal(|| false);

    // State modal ubah nama profil pengguna
    let mut is_profile_modal_open = use_signal(|| false);

    // State transaksi yang sedang diedit (jika mode edit aktif)
    let mut editing_trx = use_signal(|| None::<Transaction>);

    // State modal rincian transaksi (ketika item transaksi diklik)
    let mut selected_detail_trx = use_signal(|| None::<Transaction>);

    // State dialog konfirmasi hapus data
    let mut pending_delete = use_signal(|| None::<PendingDelete>);

    // Handler penyimpanan transaksi (menambah baru atau memperbarui hasil edit)
    let handle_save = move |saved_trx: Transaction| {
        let mut list = transactions.write();
        if let Some(pos) = list.iter().position(|t| t.id == saved_trx.id) {
            list[pos] = saved_trx;
        } else {
            list.insert(0, saved_trx);
        }
        save_transactions(&list);
        editing_trx.set(None);
        is_modal_open.set(false);
    };

    // Handler penambahan sumber dana baru
    let handle_save_wallet = move |new_w: Wallet| {
        let mut list = wallets.write();
        list.push(new_w);
        save_wallets(&list);
        is_wallet_modal_open.set(false);
    };

    // Handler penambahan kategori kustom baru
    let handle_add_category = move |(trx_t, cat_name): (TransactionType, String)| {
        let mut cats = categories.write();
        match trx_t {
            TransactionType::Expense => {
                if !cats.expense.contains(&cat_name) {
                    cats.expense.push(cat_name);
                }
            }
            TransactionType::Income => {
                if !cats.income.contains(&cat_name) {
                    cats.income.push(cat_name);
                }
            }
            TransactionType::Transfer => {}
        }
        save_categories(&cats);
    };

    // Handler toggle tema Light / Dark / Tokyo Night
    let mut toggle_theme = move || {
        let next_theme = match *theme.read() {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::TokyoNight,
            ThemeMode::TokyoNight => ThemeMode::Dark,
        };
        theme.set(next_theme);
        save_theme(next_theme);
    };

    let theme_class = match *theme.read() {
        ThemeMode::Dark => "dark",
        ThemeMode::Light => "light",
        ThemeMode::TokyoNight => "dark tokyo-night",
    };

    let theme_color_meta = match *theme.read() {
        ThemeMode::Dark => "#09090b",
        ThemeMode::Light => "#fcfcfd",
        ThemeMode::TokyoNight => "#16161e",
    };

    let is_onboarded = profile.read().as_ref().map(|p| p.is_onboarded).unwrap_or(false);

    rsx! {
        document::Title { "CatatMoney - Financial Cashflow & Expense Intelligence" }
        document::Meta { name: "description", content: "Aplikasi pencatatan keuangan pribadi minimalis, elegan, dan offline-first." }
        document::Meta { name: "theme-color", content: "{theme_color_meta}" }
        document::Meta { name: "mobile-web-app-capable", content: "yes" }
        document::Meta { name: "apple-mobile-web-app-capable", content: "yes" }
        document::Meta { name: "apple-mobile-web-app-status-bar-style", content: "black-translucent" }
        document::Meta { name: "apple-mobile-web-app-title", content: "CatatMoney" }
        document::Link { rel: "manifest", href: "/catatmoney/manifest.json" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: "/catatmoney/icon.svg" }
        document::Link { rel: "apple-touch-icon", href: "/catatmoney/icon.svg" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "true",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap",
        }

        // Sematkan stylesheet Tailwind hasil kompilasi CLI & desain sistem monokromatik
        style { "{APP_TAILWIND}" }
        style { "{APP_STYLE}" }

        // Root container yang menerapkan class dark atau light dan menangani shortcut keyboard ESC
        div {
            class: "{theme_class} min-h-screen bg-[var(--bg-app)] text-[var(--text-primary)] transition-colors duration-200 outline-none",
            tabindex: "0",
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    if pending_delete.read().is_some() {
                        pending_delete.set(None);
                    } else if *is_profile_modal_open.read() {
                        is_profile_modal_open.set(false);
                    } else if *is_wallet_modal_open.read() {
                        is_wallet_modal_open.set(false);
                    } else if *is_modal_open.read() {
                        is_modal_open.set(false);
                        editing_trx.set(None);
                    } else if selected_detail_trx.read().is_some() {
                        selected_detail_trx.set(None);
                    }
                }
            },

            if !is_onboarded {
                OnboardingWizard {
                    theme: *theme.read(),
                    on_toggle_theme: move |_| toggle_theme(),
                    on_complete: move |(new_profile, first_wallet)| {
                        save_profile(&new_profile);
                        profile.set(Some(new_profile));

                        let new_wallets = vec![first_wallet];
                        save_wallets(&new_wallets);
                        wallets.set(new_wallets);

                        save_transactions(&[]);
                        transactions.set(vec![]);

                        let default_cats = get_default_categories();
                        save_categories(&default_cats);
                        categories.set(default_cats);

                        save_budgets(&[]);
                        budgets.set(vec![]);

                        save_savings_goals(&[]);
                        savings_goals.set(vec![]);

                        save_savings_logs(&[]);
                        savings_logs.set(vec![]);
                    },
                }
            } else {
                div { class: "app-wrapper",
                    // Header Aplikasi Monokromatik Minimalis
                    header { class: "app-header",
                        div { class: "brand-section",
                            div { class: "brand-icon-box",
                                IconWallet { size: "18" }
                            }
                            div {
                                h1 { class: "brand-title", "CatatMoney" }
                                p { class: "brand-subtitle", "Financial Cashflow & Expense Intelligence" }
                            }
                        }

                        div { class: "header-controls",
                            // Tombol Profil Pengguna (Membuka Modal Ubah Nama)
                            button {
                                r#type: "button",
                                class: "profile-btn",
                                onclick: move |_| is_profile_modal_open.set(true),
                                IconUser { size: "14" }
                                span { "Profil" }
                            }
                        }
                    }

                    // Konten Berdasarkan Tab yang Aktif
                    main {
                        match *active_tab.read() {
                            NavTab::Dashboard => rsx! {
                                MainDashboard {
                                    user_name: profile.read().as_ref().map(|p| p.name.clone()).unwrap_or_else(|| "Pengguna".to_string()),
                                    transactions: transactions.read().clone(),
                                    wallets: wallets.read().clone(),
                                    budgets: budgets.read().clone(),
                                    savings_goals: savings_goals.read().clone(),
                                    on_go_to_history: move |_| active_tab.set(NavTab::History),
                                    on_select_trx: move |trx| selected_detail_trx.set(Some(trx)),
                                    on_open_add_wallet: move |_| is_wallet_modal_open.set(true),
                                    on_go_to_analytics: move |_| active_tab.set(NavTab::Analytics),
                                    on_go_to_budget: move |_| active_tab.set(NavTab::Budget),
                                    on_go_to_savings: move |_| active_tab.set(NavTab::Savings),
                                }
                            },
                            NavTab::Analytics => rsx! {
                                AnalyticsView {
                                    transactions: transactions.read().clone(),
                                    wallets: wallets.read().clone(),
                                    on_select_trx: move |trx| selected_detail_trx.set(Some(trx)),
                                }
                            },
                            NavTab::Budget => rsx! {
                                BudgetView {
                                    transactions: transactions.read().clone(),
                                    categories: categories.read().clone(),
                                    budgets: budgets.read().clone(),
                                    on_update_budgets: move |new_budgets: Vec<CategoryBudget>| {
                                        save_budgets(&new_budgets);
                                        budgets.set(new_budgets);
                                    },
                                }
                            },
                            NavTab::Savings => rsx! {
                                SavingsView {
                                    savings_goals: savings_goals.read().clone(),
                                    savings_logs: savings_logs.read().clone(),
                                    on_update_goals: move |new_goals: Vec<SavingsGoal>| {
                                        save_savings_goals(&new_goals);
                                        savings_goals.set(new_goals);
                                    },
                                    on_update_logs: move |new_logs: Vec<SavingsLogEntry>| {
                                        save_savings_logs(&new_logs);
                                        savings_logs.set(new_logs);
                                    },
                                }
                            },
                            NavTab::AiCopilot => rsx! {
                                AiCopilotView {
                                    user_name: profile.read().as_ref().map(|p| p.name.clone()).unwrap_or_else(|| "Pengguna".to_string()),
                                    gemini_api_key: gemini_api_key.read().clone(),
                                    transactions: transactions.read().clone(),
                                    wallets: wallets.read().clone(),
                                    categories: categories.read().clone(),
                                    budgets: budgets.read().clone(),
                                    savings_goals: savings_goals.read().clone(),
                                    savings_logs: savings_logs.read().clone(),
                                    on_save_api_key: move |key: String| {
                                        save_gemini_api_key(Some(&key));
                                        gemini_api_key.set(Some(key));
                                    },
                                    on_record_transaction: move |new_trx: Transaction| {
                                        let mut list = transactions.write();
                                        if let Some(pos) = list.iter().position(|t| t.id == new_trx.id) {
                                            list[pos] = new_trx;
                                        } else {
                                            list.insert(0, new_trx);
                                        }
                                        save_transactions(&list);
                                    },
                                    on_update_budgets: move |new_budgets: Vec<CategoryBudget>| {
                                        save_budgets(&new_budgets);
                                        budgets.set(new_budgets);
                                    },
                                    on_update_goals: move |new_goals: Vec<SavingsGoal>| {
                                        save_savings_goals(&new_goals);
                                        savings_goals.set(new_goals);
                                    },
                                    on_update_logs: move |new_logs: Vec<SavingsLogEntry>| {
                                        save_savings_logs(&new_logs);
                                        savings_logs.set(new_logs);
                                    },
                                    on_go_to_settings: move |_| active_tab.set(NavTab::Management),
                                }
                            },
                            NavTab::Calendar => rsx! {
                                FinancialCalendar {
                                    transactions: transactions.read().clone(),
                                    on_delete: move |trx: Transaction| {
                                        pending_delete.set(Some(PendingDelete::Transaction(trx)));
                                    },
                                    on_select_trx: move |trx| selected_detail_trx.set(Some(trx)),
                                }
                            },
                            NavTab::History => rsx! {
                                TransactionHistory {
                                    transactions: transactions.read().clone(),
                                    on_delete: move |trx: Transaction| {
                                        pending_delete.set(Some(PendingDelete::Transaction(trx)));
                                    },
                                    on_select_trx: move |trx| selected_detail_trx.set(Some(trx)),
                                }
                            },
                            NavTab::Management => rsx! {
                                ManagementView {
                                    profile: profile.read().clone(),
                                    transactions: transactions.read().clone(),
                                    wallets: wallets.read().clone(),
                                    categories: categories.read().clone(),
                                    budgets: budgets.read().clone(),
                                    savings_goals: savings_goals.read().clone(),
                                    savings_logs: savings_logs.read().clone(),
                                    gemini_api_key: gemini_api_key.read().clone(),
                                    current_theme: *theme.read(),
                                    on_change_theme: move |new_theme: ThemeMode| {
                                        theme.set(new_theme);
                                        save_theme(new_theme);
                                    },
                                    on_update_wallets: move |new_w_list: Vec<Wallet>| {
                                        save_wallets(&new_w_list);
                                        wallets.set(new_w_list);
                                    },
                                    on_update_categories: move |new_cats: UserCategories| {
                                        save_categories(&new_cats);
                                        categories.set(new_cats);
                                    },
                                    on_save_api_key: move |key: String| {
                                        save_gemini_api_key(Some(&key));
                                        gemini_api_key.set(Some(key));
                                    },
                                    on_delete_api_key: move |_| {
                                        save_gemini_api_key(None);
                                        gemini_api_key.set(None);
                                    },
                                    on_request_delete_wallet: move |w_name: String| {
                                        pending_delete.set(Some(PendingDelete::Wallet(w_name)));
                                    },
                                    on_request_delete_category: move |(t_type, cat_name): (TransactionType, String)| {
                                        pending_delete.set(Some(PendingDelete::Category {
                                            is_expense: t_type == TransactionType::Expense,
                                            name: cat_name,
                                        }));
                                    },
                                    on_restore_data: move |data: (Option<UserProfile>, Vec<Transaction>, Vec<Wallet>, UserCategories, Vec<CategoryBudget>, Vec<SavingsGoal>, Vec<SavingsLogEntry>)| {
                                        let (new_prof, new_trxs, new_wallets, new_cats, new_budgets, new_goals, new_logs) = data;
                                        if let Some(prof) = new_prof {
                                            save_profile(&prof);
                                            profile.set(Some(prof));
                                        } else if profile.read().is_none() {
                                            let def_prof = UserProfile {
                                                name: "Pengguna".to_string(),
                                                is_onboarded: true,
                                            };
                                            save_profile(&def_prof);
                                            profile.set(Some(def_prof));
                                        }
                                        save_transactions(&new_trxs);
                                        transactions.set(new_trxs);
                                        save_wallets(&new_wallets);
                                        wallets.set(new_wallets);
                                        save_categories(&new_cats);
                                        categories.set(new_cats);
                                        save_budgets(&new_budgets);
                                        budgets.set(new_budgets);
                                        save_savings_goals(&new_goals);
                                        savings_goals.set(new_goals);
                                        save_savings_logs(&new_logs);
                                        savings_logs.set(new_logs);
                                    },
                                    on_request_reset_all_data: move |_| {
                                        pending_delete.set(Some(PendingDelete::AllData));
                                    },
                                }
                            },
                        }
                    }

                    // Footer Minimalis
                    footer { class: "app-footer",
                        p { "CatatMoney • Monochrome Swiss FinTech • Tailwind CSS & Dioxus 0.7" }
                    }
                }

                // Bottom Navigation Dock & Center Floating (+) Button
                BottomNavBar {
                    active_tab: *active_tab.read(),
                    on_select_tab: move |tab| active_tab.set(tab),
                    on_open_modal: move |_| {
                        editing_trx.set(None);
                        is_modal_open.set(true);
                    },
                }

                // Modal Input & Edit Transaksi dengan Rincian Waktu & Lampiran
                TransactionModal {
                    is_open: *is_modal_open.read(),
                    editing_transaction: editing_trx.read().clone(),
                    wallets: wallets.read().clone(),
                    categories: categories.read().clone(),
                    on_close: move |_| {
                        is_modal_open.set(false);
                        editing_trx.set(None);
                    },
                    on_save: handle_save,
                    on_add_category: handle_add_category,
                    on_open_add_wallet: move |_| is_wallet_modal_open.set(true),
                }

                // Modal Tambah Sumber Dana Baru
                WalletModal {
                    is_open: *is_wallet_modal_open.read(),
                    on_close: move |_| is_wallet_modal_open.set(false),
                    on_save: handle_save_wallet,
                }

                // Modal Popup Ubah Nama Profil Pengguna
                if *is_profile_modal_open.read() {
                    ProfileModal {
                        is_open: true,
                        current_profile: profile.read().clone(),
                        on_close: move |_| is_profile_modal_open.set(false),
                        on_save: move |new_name: String| {
                            let mut current = profile.read().clone().unwrap_or(UserProfile {
                                name: new_name.clone(),
                                is_onboarded: true,
                            });
                            current.name = new_name;
                            save_profile(&current);
                            profile.set(Some(current));
                            is_profile_modal_open.set(false);
                        },
                    }
                }

                // Modal Popup Khusus untuk Rincian Transaksi
                DetailModal {
                    transaction: selected_detail_trx.read().clone(),
                    on_close: move |_| selected_detail_trx.set(None),
                    on_edit: move |trx: Transaction| {
                        selected_detail_trx.set(None);
                        editing_trx.set(Some(trx));
                        is_modal_open.set(true);
                    },
                    on_delete: move |trx: Transaction| {
                        selected_detail_trx.set(None);
                        pending_delete.set(Some(PendingDelete::Transaction(trx)));
                    },
                }
            }

            // Modal Dialog Konfirmasi Penghapusan & Reset
            if let Some(target) = pending_delete.read().clone() {
                match target {
                    PendingDelete::Transaction(trx) => rsx! {
                        ConfirmModal {
                            title: "Hapus Transaksi".to_string(),
                            message: format!("Apakah Anda yakin ingin menghapus catatan transaksi \"{}\" sebesar {}? Tindakan ini tidak dapat dibatalkan.", trx.title, format_idr(trx.amount)),
                            confirm_label: "Hapus Transaksi".to_string(),
                            on_confirm: {
                                let id = trx.id.clone();
                                move |_| {
                                    transactions.write().retain(|t| t.id != id);
                                    save_transactions(&transactions.read());
                                    pending_delete.set(None);
                                }
                            },
                            on_cancel: move |_| pending_delete.set(None),
                        }
                    },
                    PendingDelete::Wallet(w_name) => rsx! {
                        ConfirmModal {
                            title: "Hapus Sumber Dana".to_string(),
                            message: format!("Apakah Anda yakin ingin menghapus sumber dana \"{}\"? Catatan transaksi yang sudah ada tidak akan terhapus.", w_name),
                            confirm_label: "Hapus Sumber Dana".to_string(),
                            on_confirm: {
                                let name = w_name.clone();
                                move |_| {
                                    wallets.write().retain(|w| w.name != name);
                                    save_wallets(&wallets.read());
                                    pending_delete.set(None);
                                }
                            },
                            on_cancel: move |_| pending_delete.set(None),
                        }
                    },
                    PendingDelete::Category { is_expense, name } => rsx! {
                        ConfirmModal {
                            title: "Hapus Kategori".to_string(),
                            message: format!("Apakah Anda yakin ingin menghapus kategori \"{}\"? Catatan transaksi yang sudah ada tidak akan terhapus.", name),
                            confirm_label: "Hapus Kategori".to_string(),
                            on_confirm: {
                                let cat_name = name.clone();
                                move |_| {
                                    let mut cats = categories.write();
                                    if is_expense {
                                        cats.expense.retain(|c| c != &cat_name);
                                    } else {
                                        cats.income.retain(|c| c != &cat_name);
                                    }
                                    save_categories(&cats);
                                    pending_delete.set(None);
                                }
                            },
                            on_cancel: move |_| pending_delete.set(None),
                        }
                    },
                    PendingDelete::AllData => rsx! {
                        ConfirmModal {
                            title: "Reset Seluruh Data Aplikasi?".to_string(),
                            message: "Tindakan ini akan menghapus SELURUH catatan transaksi, akun sumber dana, kategori kustom, dan profil pengguna secara permanen dari perangkat ini. Apakah Anda yakin?".to_string(),
                            confirm_label: "Ya, Hapus Semua & Reset".to_string(),
                            on_confirm: move |_| {
                                reset_all_data();
                                profile.set(None);
                                transactions.set(vec![]);
                                wallets.set(vec![]);
                                categories.set(get_default_categories());
                                save_budgets(&[]);
                                budgets.set(vec![]);
                                save_savings_goals(&[]);
                                savings_goals.set(vec![]);
                                save_savings_logs(&[]);
                                savings_logs.set(vec![]);
                                pending_delete.set(None);
                            },
                            on_cancel: move |_| pending_delete.set(None),
                        }
                    },
                }
            }
        }
    }
}
