use dioxus::prelude::*;
use crate::components::icons::{
    IconAlertTriangle, IconArrowRight, IconBanknote, IconCheck, IconChevronLeft,
    IconCreditCard, IconDownload, IconLandmark, IconReceipt, IconShield,
    IconSmartphone, IconSun, IconMoon, IconWallet,
};
use crate::model::{
    generate_id, parse_input_idr, ThemeMode, UserProfile, Wallet, WalletType,
};

#[component]
pub fn OnboardingWizard(
    theme: ThemeMode,
    on_toggle_theme: EventHandler<()>,
    on_complete: EventHandler<(UserProfile, Wallet)>,
) -> Element {
    let mut step = use_signal(|| 1);
    let mut user_name = use_signal(|| String::new());

    // State Sumber Dana Awal
    let mut wallet_name = use_signal(|| "BCA".to_string());
    let mut wallet_type = use_signal(|| WalletType::Bank);
    let mut balance_display = use_signal(|| "Rp 0".to_string());
    let mut balance_val = use_signal(|| 0.0);

    let mut error_msg = use_signal(|| None::<String>);

    let handle_finish = move |_| {
        let name = user_name.read().trim().to_string();
        if name.is_empty() {
            step.set(1);
            error_msg.set(Some("Nama pengguna tidak boleh kosong.".to_string()));
            return;
        }

        let w_name = wallet_name.read().trim().to_string();
        if w_name.is_empty() {
            error_msg.set(Some("Nama sumber dana / akun tidak boleh kosong.".to_string()));
            return;
        }

        let profile = UserProfile {
            name,
            is_onboarded: true,
        };

        let first_wallet = Wallet {
            id: generate_id(),
            name: w_name,
            wallet_type: *wallet_type.read(),
            initial_balance: *balance_val.read(),
        };

        on_complete.call((profile, first_wallet));
    };

    rsx! {
        div { class: "onboarding-container",
            // Header Top Bar dengan Toggle Tema
            div { class: "onboarding-top-bar",
                div { class: "flex items-center gap-2",
                    div { class: "brand-icon-box",
                        IconWallet { size: "18" }
                    }
                    span { class: "text-sm font-bold tracking-tight text-[var(--text-primary)]", "CatatMoney" }
                }

                button {
                    r#type: "button",
                    class: "theme-toggle-btn",
                    onclick: move |_| on_toggle_theme.call(()),
                    if theme == ThemeMode::Dark {
                        IconSun { size: "14" }
                        span { "Light Mode" }
                    } else {
                        IconMoon { size: "14" }
                        span { "Dark Mode" }
                    }
                }
            }

            // Wizard Main Card
            div { class: "onboarding-card",
                // Step Indicator Pills
                div { class: "onboarding-stepper",
                    div {
                        class: if *step.read() == 1 { "stepper-item active" } else { "stepper-item done" },
                        div { class: "stepper-circle",
                            if *step.read() > 1 {
                                IconCheck { size: "12" }
                            } else {
                                span { "1" }
                            }
                        }
                        span { class: "stepper-label", "Profil Pengguna" }
                    }

                    div { class: "stepper-line" }

                    div {
                        class: if *step.read() == 2 { "stepper-item active" } else { "stepper-item" },
                        div { class: "stepper-circle", span { "2" } }
                        span { class: "stepper-label", "Akun & Saldo Awal" }
                    }
                }

                // Error Message Alert jika ada
                if let Some(ref err) = *error_msg.read() {
                    div { class: "p-3 mb-4 rounded-xl border border-[var(--negative-border)] bg-[var(--negative-bg)] text-[var(--negative)] flex items-center gap-2.5 text-xs font-medium",
                        IconAlertTriangle { size: "15" }
                        span { "{err}" }
                    }
                }

                // STEP 1: Profil Pengguna
                if *step.read() == 1 {
                    div { class: "onboarding-step-body",
                        div { class: "onboarding-header text-center mb-6",
                            h2 { class: "text-xl font-bold tracking-tight text-[var(--text-primary)]", "Selamat Datang di CatatMoney" }
                            p { class: "text-xs text-[var(--text-secondary)] mt-1.5 max-w-sm mx-auto leading-relaxed",
                                "Sistem pencatatan keuangan pribadi minimalis, berorientasi privasi, dan sepenuhnya offline tanpa server pihak ketiga."
                            }
                        }

                        div { class: "field-group mb-6",
                            label { class: "field-label", "Siapa nama panggilan Anda?" }
                            input {
                                r#type: "text",
                                class: "field-input text-sm py-2.5",
                                placeholder: "Misal: Budi, Sarah, Alex",
                                value: "{user_name}",
                                autofocus: true,
                                oninput: move |e| {
                                    user_name.set(e.value());
                                    error_msg.set(None);
                                },
                                onkeydown: move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter {
                                        let name = user_name.read().trim().to_string();
                                        if name.is_empty() {
                                            error_msg.set(Some("Silakan masukkan nama panggilan Anda untuk melanjutkan.".to_string()));
                                        } else {
                                            error_msg.set(None);
                                            step.set(2);
                                        }
                                    }
                                },
                            }
                            p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                                "Nama ini digunakan untuk personalisasi ringkasan keuangan harian Anda."
                            }
                        }

                        // Feature Highlights List
                        div { class: "onboarding-feature-grid",
                            div { class: "onboarding-feature-item",
                                div { class: "onboarding-feature-icon",
                                    IconShield { size: "16" }
                                }
                                div { class: "onboarding-feature-text",
                                    div { class: "feature-title", "100% Privat" }
                                    div { class: "feature-desc", "Tersimpan aman di peramban perangkat lokal" }
                                }
                            }
                            div { class: "onboarding-feature-item",
                                div { class: "onboarding-feature-icon",
                                    IconReceipt { size: "16" }
                                }
                                div { class: "onboarding-feature-text",
                                    div { class: "feature-title", "Arus Kas & Struk" }
                                    div { class: "feature-desc", "Catat transaksi & lampiran bukti sah" }
                                }
                            }
                            div { class: "onboarding-feature-item",
                                div { class: "onboarding-feature-icon",
                                    IconDownload { size: "16" }
                                }
                                div { class: "onboarding-feature-text",
                                    div { class: "feature-title", "Cadangan JSON" }
                                    div { class: "feature-desc", "Ekspor & pulihkan data kapan saja" }
                                }
                            }
                        }

                        div { class: "flex justify-end pt-2",
                            button {
                                r#type: "button",
                                class: "btn-primary w-full sm:w-auto text-xs py-2.5 px-6 flex items-center justify-center gap-2",
                                onclick: move |_| {
                                    let name = user_name.read().trim().to_string();
                                    if name.is_empty() {
                                        error_msg.set(Some("Silakan masukkan nama panggilan Anda untuk melanjutkan.".to_string()));
                                    } else {
                                        error_msg.set(None);
                                        step.set(2);
                                    }
                                },
                                span { "Lanjutkan ke Atur Akun" }
                                IconArrowRight { size: "14" }
                            }
                        }
                    }
                }

                // STEP 2: Atur Sumber Dana & Saldo Awal
                if *step.read() == 2 {
                    div { class: "onboarding-step-body",
                        div { class: "onboarding-header mb-5",
                            h2 { class: "text-lg font-bold tracking-tight text-[var(--text-primary)]", "Atur Sumber Dana Pertama" }
                            p { class: "text-xs text-[var(--text-secondary)] mt-1 leading-relaxed",
                                "Tentukan rekening atau dompet utama Anda. Saldo awal ini menjadi fondasi pencatatan arus kas Anda."
                            }
                        }

                        // Presets Cepat
                        div { class: "mb-4",
                            span { class: "text-[11px] font-semibold text-[var(--text-muted)] uppercase tracking-wider block mb-2", "Pilihan Cepat" }
                            div { class: "flex flex-wrap gap-1.5",
                                button {
                                    r#type: "button",
                                    class: "preset-pill",
                                    onclick: move |_| {
                                        wallet_name.set("BCA".to_string());
                                        wallet_type.set(WalletType::Bank);
                                        error_msg.set(None);
                                    },
                                    "BCA"
                                }
                                button {
                                    r#type: "button",
                                    class: "preset-pill",
                                    onclick: move |_| {
                                        wallet_name.set("Bank Mandiri".to_string());
                                        wallet_type.set(WalletType::Bank);
                                        error_msg.set(None);
                                    },
                                    "Mandiri"
                                }
                                button {
                                    r#type: "button",
                                    class: "preset-pill",
                                    onclick: move |_| {
                                        wallet_name.set("GoPay".to_string());
                                        wallet_type.set(WalletType::EWallet);
                                        error_msg.set(None);
                                    },
                                    "GoPay"
                                }
                                button {
                                    r#type: "button",
                                    class: "preset-pill",
                                    onclick: move |_| {
                                        wallet_name.set("DANA".to_string());
                                        wallet_type.set(WalletType::EWallet);
                                        error_msg.set(None);
                                    },
                                    "DANA"
                                }
                                button {
                                    r#type: "button",
                                    class: "preset-pill",
                                    onclick: move |_| {
                                        wallet_name.set("Uang Dompet".to_string());
                                        wallet_type.set(WalletType::Cash);
                                        error_msg.set(None);
                                    },
                                    "Uang Tunai"
                                }
                            }
                        }

                        // Form Sumber Dana
                        div { class: "field-group mb-3",
                            label { class: "field-label", "Nama Sumber Dana / Akun" }
                            input {
                                r#type: "text",
                                class: "field-input text-xs",
                                placeholder: "Misal: BCA, GoPay, Kas Dompet",
                                value: "{wallet_name}",
                                oninput: move |e| {
                                    wallet_name.set(e.value());
                                    error_msg.set(None);
                                },
                            }
                        }

                        div { class: "field-group mb-3.5",
                            label { class: "field-label", "Tipe Akun" }
                            div { class: "wallet-type-grid",
                                button {
                                    r#type: "button",
                                    class: if *wallet_type.read() == WalletType::Bank { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                    onclick: move |_| wallet_type.set(WalletType::Bank),
                                    IconLandmark { size: "15" }
                                    span { "Rekening Bank" }
                                }
                                button {
                                    r#type: "button",
                                    class: if *wallet_type.read() == WalletType::EWallet { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                    onclick: move |_| wallet_type.set(WalletType::EWallet),
                                    IconSmartphone { size: "15" }
                                    span { "E-Wallet" }
                                }
                                button {
                                    r#type: "button",
                                    class: if *wallet_type.read() == WalletType::Cash { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                    onclick: move |_| wallet_type.set(WalletType::Cash),
                                    IconBanknote { size: "15" }
                                    span { "Uang Tunai" }
                                }
                                button {
                                    r#type: "button",
                                    class: if *wallet_type.read() == WalletType::CreditCard { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                    onclick: move |_| wallet_type.set(WalletType::CreditCard),
                                    IconCreditCard { size: "15" }
                                    span { "Kartu Kredit" }
                                }
                                button {
                                    r#type: "button",
                                    class: if *wallet_type.read() == WalletType::Other { "wallet-type-select-btn active" } else { "wallet-type-select-btn" },
                                    onclick: move |_| wallet_type.set(WalletType::Other),
                                    IconWallet { size: "15" }
                                    span { "Lainnya" }
                                }
                            }
                        }

                        div { class: "field-group mb-6",
                            label { class: "field-label", "Saldo Awal Saat Ini" }
                            input {
                                r#type: "text",
                                class: "field-input text-sm tabular-numbers",
                                placeholder: "Rp 0",
                                value: "{balance_display}",
                                oninput: move |e| {
                                    let (val, formatted) = parse_input_idr(&e.value());
                                    balance_val.set(val);
                                    if val == 0.0 {
                                        balance_display.set("Rp 0".to_string());
                                    } else {
                                        balance_display.set(format!("Rp {}", formatted));
                                    }
                                },
                            }
                            p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                                "Masukkan saldo nyata akun Anda saat ini. Kosongkan atau biarkan Rp 0 jika belum ada saldo."
                            }
                        }

                        // Tombol Aksi
                        div { class: "flex items-center justify-between pt-3 border-t border-[var(--border-subtle)]",
                            button {
                                r#type: "button",
                                class: "btn-secondary text-xs py-2 px-4 flex items-center gap-1.5",
                                onclick: move |_| {
                                    error_msg.set(None);
                                    step.set(1);
                                },
                                IconChevronLeft { size: "14" }
                                "Kembali"
                            }
                            button {
                                r#type: "button",
                                class: "btn-primary text-xs py-2.5 px-5 flex items-center gap-2",
                                onclick: handle_finish,
                                IconCheck { size: "14" }
                                "Selesaikan & Buka Aplikasi"
                            }
                        }
                    }
                }
            }
        }
    }
}
