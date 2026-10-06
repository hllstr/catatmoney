use dioxus::prelude::*;
use crate::audio::sleep_ms;
use crate::components::icons::{
    IconBanknote, IconCheck, IconCreditCard, IconLandmark, IconPlus, IconSmartphone, IconWallet, IconX,
};
use crate::model::{format_idr, generate_id, parse_input_idr, Wallet, WalletType};

#[component]
pub fn WalletModal(
    is_open: bool,
    #[props(default = None)] initial_wallet: Option<Wallet>,
    on_close: EventHandler<()>,
    on_save: EventHandler<Wallet>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let is_edit = initial_wallet.is_some();
    let init_w = initial_wallet.clone();
    let init_name = init_w.as_ref().map(|w| w.name.clone()).unwrap_or_default();
    let init_type = init_w.as_ref().map(|w| w.wallet_type).unwrap_or(WalletType::Bank);
    let init_balance = init_w.as_ref().map(|w| w.initial_balance).unwrap_or(0.0);
    let init_display = if init_balance == 0.0 {
        "Rp 0".to_string()
    } else {
        format_idr(init_balance)
    };

    let mut name = use_signal(move || init_name.clone());
    let mut wallet_type = use_signal(move || init_type);
    let mut balance_val = use_signal(move || init_balance);
    let mut balance_display = use_signal(move || init_display.clone());
    let mut error_msg = use_signal(|| None::<String>);
    let mut is_closing = use_signal(|| false);

    let handle_close = move || {
        if *is_closing.read() {
            return;
        }
        is_closing.set(true);
        let on_close = on_close.clone();
        spawn(async move {
            sleep_ms(220).await;
            is_closing.set(false);
            on_close.call(());
        });
    };

    let handle_submit = {
        let on_save = on_save.clone();
        let on_close = on_close.clone();
        let initial_wallet = initial_wallet.clone();
        move |evt: FormEvent| {
            evt.prevent_default();

            let n = name.read().trim().to_string();
            if n.is_empty() {
                error_msg.set(Some("Nama sumber dana wajib diisi.".to_string()));
                return;
            }

            let wallet_id = match &initial_wallet {
                Some(w) => w.id.clone(),
                None => generate_id(),
            };

            let new_wallet = Wallet {
                id: wallet_id,
                name: n,
                wallet_type: *wallet_type.read(),
                initial_balance: *balance_val.read(),
            };

            error_msg.set(None);
            if *is_closing.read() {
                return;
            }
            is_closing.set(true);
            let on_save = on_save.clone();
            let on_close = on_close.clone();
            spawn(async move {
                on_save.call(new_wallet);
                sleep_ms(220).await;
                is_closing.set(false);
                on_close.call(());
            });
        }
    };

    let closing_class = if *is_closing.read() { "modal-closing" } else { "" };

    rsx! {
        div {
            class: "modal-backdrop {closing_class}",
            onclick: {
                let mut close_fn = handle_close.clone();
                move |_| close_fn()
            },
            div {
                class: "modal-dialog max-w-md {closing_class}",
                onclick: move |e| e.stop_propagation(),

                div { class: "modal-header",
                    div {
                        h2 { class: "modal-title flex items-center gap-2",
                            IconWallet { size: "18" }
                            if is_edit { "Edit Sumber Dana" } else { "Tambah Sumber Dana Baru" }
                        }
                        p { class: "modal-subtitle",
                            if is_edit {
                                "Perbarui nama, jenis akun, atau saldo awal sumber dana Anda"
                            } else {
                                "Daftarkan rekening bank, e-wallet, atau pos kas Anda"
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "btn-icon-close",
                        onclick: {
                            let mut close_fn = handle_close.clone();
                            move |_| close_fn()
                        },
                        IconX { size: "18" }
                    }
                }

                if let Some(err) = error_msg.read().as_ref() {
                    div { class: "form-error mb-3",
                        "{err}"
                    }
                }

                form { onsubmit: handle_submit,
                    // Nama Sumber Dana
                    div { class: "field-group mb-4",
                        label { class: "field-label", "Nama Sumber Dana / Akun" }
                        input {
                            class: "field-input",
                            r#type: "text",
                            placeholder: "Contoh: BCA Prioritas, Mandiri, GoPay, Kas Dompet",
                            value: "{name}",
                            oninput: move |e| {
                                name.set(e.value());
                                error_msg.set(None);
                            },
                            autofocus: true,
                        }
                    }

                    // Tipe Sumber Dana
                    div { class: "field-group mb-4",
                        label { class: "field-label", "Jenis Sumber Dana" }
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

                    // Saldo Awal Saat Ini
                    div { class: "field-group mb-4",
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
                            "Saldo nyata akun ini saat pertama kali didaftarkan. Kosongkan atau biarkan Rp 0 jika belum ada saldo."
                        }
                    }

                    // Tombol Aksi
                    div { class: "flex items-center justify-end gap-2 pt-3 border-t border-[var(--border-color)]",
                        button {
                            r#type: "button",
                            class: "btn-secondary py-2 px-4 text-xs",
                            onclick: {
                                let mut close_fn = handle_close.clone();
                                move |_| close_fn()
                            },
                            "Batal"
                        }
                        button {
                            r#type: "submit",
                            class: "btn-primary py-2 px-4 text-xs flex items-center gap-1.5",
                            if is_edit {
                                IconCheck { size: "14" }
                            } else {
                                IconPlus { size: "14" }
                            }
                            if is_edit { "Simpan Perubahan" } else { "Simpan Sumber Dana" }
                        }
                    }
                }
            }
        }
    }
}
