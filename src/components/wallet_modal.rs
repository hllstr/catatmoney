use dioxus::prelude::*;
use crate::components::icons::{
    IconBanknote, IconCreditCard, IconLandmark, IconPlus, IconSmartphone, IconWallet, IconX,
};
use crate::model::{generate_id, parse_input_idr, Wallet, WalletType};

#[component]
pub fn WalletModal(
    is_open: bool,
    on_close: EventHandler<()>,
    on_save: EventHandler<Wallet>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let mut name = use_signal(String::new);
    let mut wallet_type = use_signal(|| WalletType::Bank);
    let mut balance_val = use_signal(|| 0.0);
    let mut balance_display = use_signal(|| "Rp 0".to_string());
    let mut error_msg = use_signal(|| None::<String>);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let n = name.read().trim().to_string();
        if n.is_empty() {
            error_msg.set(Some("Nama sumber dana wajib diisi.".to_string()));
            return;
        }

        let new_wallet = Wallet {
            id: generate_id(),
            name: n,
            wallet_type: *wallet_type.read(),
            initial_balance: *balance_val.read(),
        };

        on_save.call(new_wallet);
        name.set(String::new());
        balance_val.set(0.0);
        balance_display.set("Rp 0".to_string());
        error_msg.set(None);
    };

    rsx! {
        div { class: "modal-backdrop", onclick: move |_| on_close.call(()),
            div {
                class: "modal-dialog max-w-md",
                onclick: move |e| e.stop_propagation(),

                div { class: "modal-header",
                    div {
                        h2 { class: "modal-title flex items-center gap-2",
                            IconWallet { size: "18" }
                            "Tambah Sumber Dana Baru"
                        }
                        p { class: "modal-subtitle", "Daftarkan rekening bank, e-wallet, atau pos kas Anda" }
                    }
                    button {
                        r#type: "button",
                        class: "btn-icon-close",
                        onclick: move |_| on_close.call(()),
                        IconX { size: "18" }
                    }
                }

                if let Some(err) = error_msg.read().as_ref() {
                    div { class: "form-error",
                        "{err}"
                    }
                }

                form { onsubmit: handle_submit,
                    // Nama Sumber Dana
                    div { class: "field-group",
                        label { class: "field-label", "Nama Sumber Dana / Akun" }
                        input {
                            class: "field-input",
                            r#type: "text",
                            placeholder: "Contoh: BCA Prioritas, Mandiri, GoPay, Kas Dompet",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                            autofocus: true,
                        }
                    }

                    // Tipe Sumber Dana
                    div { class: "field-group",
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
                    div { class: "field-group",
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
                            onclick: move |_| on_close.call(()),
                            "Batal"
                        }
                        button {
                            r#type: "submit",
                            class: "btn-primary py-2 px-4 text-xs flex items-center gap-1.5",
                            IconPlus { size: "14" }
                            "Simpan Sumber Dana"
                        }
                    }
                }
            }
        }
    }
}
