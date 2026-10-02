use dioxus::prelude::*;
use crate::components::icons::{IconArrowDownRight, IconArrowUpRight, IconPlus};
use crate::model::{
    generate_id, Transaction, TransactionType, EXPENSE_CATEGORIES, INCOME_CATEGORIES,
};

#[allow(dead_code)]
fn get_today_date() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let year = date.get_full_year();
        let month = date.get_month() + 1;
        let day = date.get_date();
        format!("{:04}-{:02}-{:02}", year, month, day)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "2026-10-02".to_string()
    }
}

#[component]
pub fn TransactionForm(on_add: EventHandler<Transaction>) -> Element {
    let mut trx_type = use_signal(|| TransactionType::Expense);
    let mut title = use_signal(String::new);
    let mut amount_str = use_signal(String::new);
    let mut category = use_signal(|| EXPENSE_CATEGORIES[0].to_string());
    let mut date = use_signal(get_today_date);
    let mut notes = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);

    let current_categories = match *trx_type.read() {
        TransactionType::Expense => EXPENSE_CATEGORIES,
        TransactionType::Income => INCOME_CATEGORIES,
    };

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let t = title.read().trim().to_string();
        if t.is_empty() {
            error_msg.set(Some("Judul transaksi wajib diisi.".to_string()));
            return;
        }

        let a_str = amount_str.read().replace('.', "").replace(',', ".");
        let a = match a_str.trim().parse::<f64>() {
            Ok(val) if val > 0.0 => val,
            _ => {
                error_msg.set(Some("Nominal uang harus berupa angka positif.".to_string()));
                return;
            }
        };

        let d = date.read().trim().to_string();
        if d.is_empty() {
            error_msg.set(Some("Tanggal transaksi wajib ditentukan.".to_string()));
            return;
        }

        let new_trx = Transaction {
            id: generate_id(),
            title: t,
            amount: a,
            transaction_type: *trx_type.read(),
            category: category.read().clone(),
            wallet: "BCA".to_string(),
            date: d,
            time: crate::model::get_current_time(),
            notes: notes.read().trim().to_string(),
            attachment_name: None,
            attachment_size: None,
            attachment_data: None,
        };

        on_add.call(new_trx);

        // Reset form
        title.set(String::new());
        amount_str.set(String::new());
        notes.set(String::new());
        error_msg.set(None);
    };

    rsx! {
        div { class: "surface-panel",
            div { class: "panel-header",
                h2 { class: "panel-title",
                    IconPlus { size: "16" }
                    "Transaksi Baru"
                }
            }

            // Segmented Control untuk Tipe Transaksi
            div { class: "segmented-control",
                button {
                    r#type: "button",
                    class: if *trx_type.read() == TransactionType::Expense { "segment-btn active-expense" } else { "segment-btn" },
                    onclick: move |_| {
                        trx_type.set(TransactionType::Expense);
                        category.set(EXPENSE_CATEGORIES[0].to_string());
                    },
                    IconArrowDownRight { size: "14" }
                    "Pengeluaran"
                }
                button {
                    r#type: "button",
                    class: if *trx_type.read() == TransactionType::Income { "segment-btn active-income" } else { "segment-btn" },
                    onclick: move |_| {
                        trx_type.set(TransactionType::Income);
                        category.set(INCOME_CATEGORIES[0].to_string());
                    },
                    IconArrowUpRight { size: "14" }
                    "Pemasukan"
                }
            }

            if let Some(err) = error_msg.read().as_ref() {
                div { class: "form-error",
                    "{err}"
                }
            }

            form { onsubmit: handle_submit,
                div { class: "field-group",
                    label { class: "field-label", "Deskripsi Transaksi" }
                    input {
                        class: "field-input",
                        r#type: "text",
                        placeholder: if *trx_type.read() == TransactionType::Expense { "Contoh: Langganan Cloud, Groceries" } else { "Contoh: Gaji Pokok, Penjualan" },
                        value: "{title}",
                        oninput: move |e| title.set(e.value()),
                    }
                }

                div { class: "field-group",
                    label { class: "field-label", "Nominal (IDR)" }
                    input {
                        class: "field-input tabular-numbers",
                        r#type: "number",
                        min: "1",
                        step: "any",
                        placeholder: "0",
                        value: "{amount_str}",
                        oninput: move |e| amount_str.set(e.value()),
                    }
                }

                div { class: "field-group",
                    label { class: "field-label", "Kategori" }
                    select {
                        class: "field-select",
                        value: "{category}",
                        onchange: move |e| category.set(e.value()),
                        for cat in current_categories {
                            option { value: "{cat}", "{cat}" }
                        }
                    }
                }

                div { class: "field-group",
                    label { class: "field-label", "Tanggal" }
                    input {
                        class: "field-input",
                        r#type: "date",
                        value: "{date}",
                        oninput: move |e| date.set(e.value()),
                    }
                }

                div { class: "field-group",
                    label { class: "field-label", "Catatan Detail (Opsional)" }
                    textarea {
                        class: "field-textarea",
                        placeholder: "Tambahkan detail atau keterangan pengingat...",
                        value: "{notes}",
                        oninput: move |e| notes.set(e.value()),
                    }
                }

                button {
                    r#type: "submit",
                    class: "btn-primary",
                    IconPlus { size: "15" }
                    "Catat Transaksi"
                }
            }
        }
    }
}
