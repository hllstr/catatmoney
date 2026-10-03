use dioxus::prelude::*;
use crate::components::icons::{
    IconArrowDownRight, IconArrowLeftRight, IconArrowRight, IconArrowUpRight, IconCalendar,
    IconClock, IconPaperclip, IconPlus, IconWallet, IconX,
};
use crate::model::{
    format_number_dots, generate_id, get_current_time_hm, get_today_date, parse_input_idr,
    Transaction, TransactionType, UserCategories, Wallet,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[component]
pub fn TransactionModal(
    is_open: bool,
    editing_transaction: Option<Transaction>,
    wallets: Vec<Wallet>,
    categories: UserCategories,
    on_close: EventHandler<()>,
    on_save: EventHandler<Transaction>,
    on_add_category: EventHandler<(TransactionType, String)>,
    on_open_add_wallet: EventHandler<()>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let is_edit = editing_transaction.is_some();
    let edit_id = editing_transaction.as_ref().map(|t| t.id.clone());

    let mut trx_type = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| t.transaction_type)
            .unwrap_or(TransactionType::Expense)
    });
    let mut title = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| t.title.clone())
            .unwrap_or_default()
    });
    let mut amount_str = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| format_number_dots(t.amount.round() as u64))
            .unwrap_or_default()
    });
    let mut amount_raw = use_signal(|| {
        editing_transaction.as_ref().map(|t| t.amount).unwrap_or(0.0)
    });

    let current_cat_list = match *trx_type.read() {
        TransactionType::Expense => categories.expense.clone(),
        TransactionType::Income => categories.income.clone(),
        TransactionType::Transfer => vec!["Transfer Internal".to_string()],
    };
    let default_cat = current_cat_list.first().cloned().unwrap_or_else(|| "Lainnya".to_string());
    let mut category = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| t.category.clone())
            .unwrap_or(default_cat)
    });

    let default_wallet_name = wallets.first().map(|w| w.name.clone()).unwrap_or_else(|| "BCA".to_string());
    let mut wallet = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| t.wallet.clone())
            .unwrap_or(default_wallet_name.clone())
    });

    let default_to_wallet = wallets
        .iter()
        .find(|w| w.name != default_wallet_name)
        .map(|w| w.name.clone())
        .or_else(|| wallets.first().map(|w| w.name.clone()))
        .unwrap_or_else(|| "GoPay".to_string());

    let mut to_wallet = use_signal(|| {
        editing_transaction
            .as_ref()
            .and_then(|t| t.to_wallet.clone())
            .unwrap_or(default_to_wallet)
    });

    let mut admin_fee_str = use_signal(|| {
        editing_transaction
            .as_ref()
            .and_then(|t| t.admin_fee)
            .filter(|&f| f > 0.0)
            .map(|f| format_number_dots(f.round() as u64))
            .unwrap_or_default()
    });
    let mut admin_fee_raw = use_signal(|| {
        editing_transaction
            .as_ref()
            .and_then(|t| t.admin_fee)
            .unwrap_or(0.0)
    });

    let mut show_new_category_input = use_signal(|| false);
    let mut new_category_name = use_signal(String::new);
    let mut date = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| t.date.clone())
            .unwrap_or_else(get_today_date)
    });
    let mut time = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| {
                let tm = t.time.trim();
                if tm.len() >= 5 {
                    tm[..5].to_string()
                } else {
                    tm.to_string()
                }
            })
            .unwrap_or_else(get_current_time_hm)
    });
    let mut notes = use_signal(|| {
        editing_transaction
            .as_ref()
            .map(|t| t.notes.clone())
            .unwrap_or_default()
    });
    let mut attachment_name = use_signal(|| {
        editing_transaction
            .as_ref()
            .and_then(|t| t.attachment_name.clone())
    });
    let mut attachment_size = use_signal(|| {
        editing_transaction
            .as_ref()
            .and_then(|t| t.attachment_size.clone())
    });
    let mut attachment_data = use_signal(|| {
        editing_transaction
            .as_ref()
            .and_then(|t| t.attachment_data.clone())
    });
    let mut error_msg = use_signal(|| None::<String>);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let t = title.read().trim().to_string();
        if t.is_empty() {
            error_msg.set(Some("Deskripsi transaksi wajib diisi.".to_string()));
            return;
        }

        let a = *amount_raw.read();
        if a <= 0.0 {
            error_msg.set(Some("Nominal uang harus lebih besar dari 0.".to_string()));
            return;
        }

        let current_type = *trx_type.read();
        let from_w = wallet.read().trim().to_string();
        let dest_w = to_wallet.read().trim().to_string();

        if current_type == TransactionType::Transfer && from_w == dest_w {
            error_msg.set(Some("Akun asal dan akun tujuan tidak boleh sama.".to_string()));
            return;
        }

        let d = date.read().trim().to_string();
        if d.is_empty() {
            error_msg.set(Some("Tanggal transaksi wajib ditentukan.".to_string()));
            return;
        }

        let tm = time.read().trim().to_string();
        let time_val = if tm.is_empty() { get_current_time_hm() } else { tm };

        let cat_val = if current_type == TransactionType::Transfer {
            "Transfer Internal".to_string()
        } else {
            category.read().clone()
        };

        let target_to_wallet = if current_type == TransactionType::Transfer {
            Some(dest_w)
        } else {
            None
        };

        let target_admin_fee = if current_type == TransactionType::Transfer {
            let fee = *admin_fee_raw.read();
            if fee > 0.0 { Some(fee) } else { None }
        } else {
            None
        };

        let target_id = edit_id.clone().unwrap_or_else(generate_id);

        let saved_trx = Transaction {
            id: target_id,
            title: t,
            amount: a,
            transaction_type: current_type,
            category: cat_val,
            wallet: from_w,
            to_wallet: target_to_wallet,
            admin_fee: target_admin_fee,
            date: d,
            time: time_val,
            notes: notes.read().trim().to_string(),
            attachment_name: attachment_name.read().clone(),
            attachment_size: attachment_size.read().clone(),
            attachment_data: attachment_data.read().clone(),
        };

        on_save.call(saved_trx);
        on_close.call(());
    };

    rsx! {
        // Modal Backdrop
        div { class: "modal-backdrop", onclick: move |_| on_close.call(()),
            div {
                class: "modal-dialog",
                onclick: move |e| e.stop_propagation(),

                div { class: "modal-header",
                    div {
                        h2 { class: "modal-title",
                            if is_edit { "Perbarui Transaksi" } else { "Catat Transaksi" }
                        }
                        p { class: "modal-subtitle",
                            if is_edit { "Ubah data nominal, kategori, waktu, atau lampiran" } else { "Input transaksi dengan format IDR otomatis & lampiran berkas" }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "btn-icon-close",
                        onclick: move |_| on_close.call(()),
                        IconX { size: "18" }
                    }
                }

                // Segmented Type Selector
                div { class: "segmented-control",
                    button {
                        r#type: "button",
                        class: if *trx_type.read() == TransactionType::Expense { "segment-btn active-expense" } else { "segment-btn" },
                        onclick: {
                            let exp_def = categories.expense.first().cloned().unwrap_or_else(|| "Lainnya".to_string());
                            move |_| {
                                trx_type.set(TransactionType::Expense);
                                category.set(exp_def.clone());
                            }
                        },
                        IconArrowDownRight { size: "14" }
                        "Pengeluaran"
                    }
                    button {
                        r#type: "button",
                        class: if *trx_type.read() == TransactionType::Income { "segment-btn active-income" } else { "segment-btn" },
                        onclick: {
                            let inc_def = categories.income.first().cloned().unwrap_or_else(|| "Lainnya".to_string());
                            move |_| {
                                trx_type.set(TransactionType::Income);
                                category.set(inc_def.clone());
                            }
                        },
                        IconArrowUpRight { size: "14" }
                        "Pemasukan"
                    }
                    button {
                        r#type: "button",
                        class: if *trx_type.read() == TransactionType::Transfer { "segment-btn active-transfer" } else { "segment-btn" },
                        onclick: move |_| {
                            trx_type.set(TransactionType::Transfer);
                            category.set("Transfer Internal".to_string());
                        },
                        IconArrowLeftRight { size: "14" }
                        "Transfer"
                    }
                }

                if let Some(err) = error_msg.read().as_ref() {
                    div { class: "form-error",
                        "{err}"
                    }
                }

                form { onsubmit: handle_submit,
                    // Deskripsi
                    div { class: "field-group",
                        label { class: "field-label", "Deskripsi Transaksi" }
                        input {
                            class: "field-input",
                            r#type: "text",
                            placeholder: match *trx_type.read() {
                                TransactionType::Expense => "Contoh: Belanja Bulanan, Paket Internet",
                                TransactionType::Income => "Contoh: Gaji, Project Freelance",
                                TransactionType::Transfer => "Contoh: Top Up GoPay, Tarik Tunai BCA",
                            },
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                        }
                    }

                    // Nominal IDR dengan Live Formatting
                    div { class: "field-group",
                        label { class: "field-label", "Nominal (IDR)" }
                        div { class: "currency-input-box",
                            span { class: "currency-prefix tabular-numbers", "Rp" }
                            input {
                                class: "field-input currency-input tabular-numbers",
                                r#type: "text",
                                inputmode: "numeric",
                                placeholder: "0",
                                value: "{amount_str}",
                                oninput: move |e| {
                                    let (num, formatted) = parse_input_idr(&e.value());
                                    amount_raw.set(num);
                                    amount_str.set(formatted);
                                },
                            }
                        }
                    }

                    if *trx_type.read() == TransactionType::Transfer {
                        // Transfer: Pilihan Akun Asal & Akun Tujuan
                        div { class: "field-group",
                            div { class: "flex items-center justify-between mb-1",
                                label { class: "field-label mb-0",
                                    span { class: "flex items-center gap-1.5",
                                        IconWallet { size: "13" }
                                        "Alur Perpindahan Saldo"
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "btn-link text-xs flex items-center gap-1",
                                    onclick: move |_| on_open_add_wallet.call(()),
                                    IconPlus { size: "12" }
                                    "Akun Baru"
                                }
                            }

                            div { class: "transfer-wallets-grid",
                                div {
                                    label { class: "text-[11px] font-semibold text-[var(--text-muted)] mb-1 block", "Dari Akun (Asal)" }
                                    select {
                                        class: "field-select text-xs py-2",
                                        value: "{wallet}",
                                        onchange: move |e| wallet.set(e.value()),
                                        for w in wallets.iter() {
                                            option { value: "{w.name}", "{w.name}" }
                                        }
                                    }
                                }
                                div { class: "transfer-arrow-separator",
                                    IconArrowRight { size: "14" }
                                }
                                div {
                                    label { class: "text-[11px] font-semibold text-[var(--text-muted)] mb-1 block", "Ke Akun (Tujuan)" }
                                    select {
                                        class: "field-select text-xs py-2",
                                        value: "{to_wallet}",
                                        onchange: move |e| to_wallet.set(e.value()),
                                        for w in wallets.iter() {
                                            option { value: "{w.name}", "{w.name}" }
                                        }
                                    }
                                }
                            }
                        }

                        // Biaya Admin Transfer (Opsional)
                        div { class: "field-group",
                            div { class: "flex items-center justify-between mb-1",
                                label { class: "field-label mb-0", "Biaya Admin (Opsional)" }
                                span { class: "text-[11px] text-[var(--text-muted)]", "Dipotong dari akun asal" }
                            }
                            div { class: "currency-input-box",
                                span { class: "currency-prefix tabular-numbers", "Rp" }
                                input {
                                    class: "field-input currency-input tabular-numbers",
                                    r#type: "text",
                                    inputmode: "numeric",
                                    placeholder: "0",
                                    value: "{admin_fee_str}",
                                    oninput: move |e| {
                                        let (num, formatted) = parse_input_idr(&e.value());
                                        admin_fee_raw.set(num);
                                        admin_fee_str.set(formatted);
                                    },
                                }
                            }
                        }
                    } else {
                        // Sumber Dana / Akun
                        div { class: "field-group",
                            div { class: "flex items-center justify-between mb-1",
                                label { class: "field-label mb-0",
                                    span { class: "flex items-center gap-1.5",
                                        IconWallet { size: "13" }
                                        "Sumber Dana / Akun"
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "btn-link text-xs flex items-center gap-1",
                                    onclick: move |_| on_open_add_wallet.call(()),
                                    IconPlus { size: "12" }
                                    "Akun Baru"
                                }
                            }
                            select {
                                class: "field-select",
                                value: "{wallet}",
                                onchange: move |e| wallet.set(e.value()),
                                for w in wallets.iter() {
                                    option { value: "{w.name}", "{w.name} ({w.wallet_type.as_str()})" }
                                }
                            }
                        }

                        // Kategori
                        div { class: "field-group",
                            div { class: "flex items-center justify-between mb-1",
                                label { class: "field-label mb-0", "Kategori" }
                                button {
                                    r#type: "button",
                                    class: "btn-link text-xs flex items-center gap-1",
                                    onclick: move |_| {
                                        let curr = *show_new_category_input.read();
                                        show_new_category_input.set(!curr);
                                    },
                                    IconPlus { size: "12" }
                                    if *show_new_category_input.read() { "Tutup" } else { "Kategori Kustom" }
                                }
                            }

                            if *show_new_category_input.read() {
                                div { class: "inline-add-category-box mb-2 p-2 rounded-lg border border-[var(--border-color)] bg-[var(--bg-card)] flex items-center gap-2",
                                    input {
                                        class: "field-input text-xs py-1.5",
                                        r#type: "text",
                                        placeholder: "Nama kategori kustom baru...",
                                        value: "{new_category_name}",
                                        oninput: move |e| new_category_name.set(e.value()),
                                    }
                                    button {
                                        r#type: "button",
                                        class: "btn-primary text-xs py-1.5 px-3 flex items-center gap-1 whitespace-nowrap",
                                        onclick: move |_| {
                                            let cat_text = new_category_name.read().trim().to_string();
                                            if !cat_text.is_empty() {
                                                on_add_category.call((*trx_type.read(), cat_text.clone()));
                                                category.set(cat_text);
                                                new_category_name.set(String::new());
                                                show_new_category_input.set(false);
                                            }
                                        },
                                        IconPlus { size: "12" }
                                        "Tambah"
                                    }
                                }
                            }

                            select {
                                class: "field-select",
                                value: "{category}",
                                onchange: move |e| category.set(e.value()),
                                for cat in current_cat_list.iter() {
                                    option { value: "{cat}", "{cat}" }
                                }
                            }
                        }
                    }

                    // Tanggal & Waktu (Native Date & Clock Picker)
                    div { class: "grid grid-cols-2 gap-3 mb-4",
                        div { class: "field-group mb-0",
                            div { class: "flex items-center justify-between mb-1",
                                label { class: "field-label mb-0",
                                    span { class: "flex items-center gap-1.5",
                                        IconCalendar { size: "12" }
                                        "Tanggal"
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "btn-link text-[11px] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors",
                                    onclick: move |_| {
                                        date.set(get_today_date());
                                    },
                                    "Hari Ini"
                                }
                            }
                            input {
                                class: "field-input tabular-numbers",
                                r#type: "date",
                                value: "{date}",
                                oninput: move |e| date.set(e.value()),
                            }
                        }
                        div { class: "field-group mb-0",
                            div { class: "flex items-center justify-between mb-1",
                                label { class: "field-label mb-0",
                                    span { class: "flex items-center gap-1.5",
                                        IconClock { size: "12" }
                                        "Waktu"
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "btn-link text-[11px] text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors",
                                    onclick: move |_| {
                                        time.set(get_current_time_hm());
                                    },
                                    "Sekarang"
                                }
                            }
                            input {
                                class: "field-input tabular-numbers",
                                r#type: "time",
                                value: "{time}",
                                oninput: move |e| time.set(e.value()),
                            }
                        }
                    }

                    // Lampiran Bukti / Struk (Attachment)
                    div { class: "field-group",
                        label { class: "field-label flex items-center justify-between",
                            span { class: "flex items-center gap-1",
                                IconPaperclip { size: "12" }
                                "Lampiran Bukti / Struk"
                            }
                            if attachment_name.read().is_some() {
                                button {
                                    r#type: "button",
                                    class: "text-xs text-negative hover:underline",
                                    onclick: move |_| {
                                        attachment_name.set(None);
                                        attachment_size.set(None);
                                        attachment_data.set(None);
                                    },
                                    "Hapus File"
                                }
                            }
                        }

                        if let Some(name) = attachment_name.read().as_ref() {
                            div { class: "attachment-preview-box",
                                div { class: "flex items-center gap-2",
                                    IconPaperclip { size: "14" }
                                    span { class: "font-semibold text-xs", "{name}" }
                                    if let Some(sz) = attachment_size.read().as_ref() {
                                        span { class: "text-xs text-muted", "({sz})" }
                                    }
                                }
                                if attachment_data.read().is_some() {
                                    span { class: "text-xs text-positive font-semibold", "Siap Ditampilkan" }
                                }
                            }
                        } else {
                            div { class: "attachment-upload-zone",
                                input {
                                    r#type: "file",
                                    id: "receipt-file-input",
                                    accept: "image/*,.pdf",
                                    class: "hidden",
                                    onchange: move |_e| {
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(window) = web_sys::window() {
                                                if let Some(doc) = window.document() {
                                                    if let Some(el) = doc.get_element_by_id("receipt-file-input") {
                                                        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                                                            if let Some(files) = input.files() {
                                                                if let Some(file) = files.get(0) {
                                                                    let name = file.name();
                                                                    let size_kb = (file.size() as f64 / 1024.0).round() as u64;
                                                                    let size_str = format!("{} KB", size_kb);
                                                                    attachment_name.set(Some(name));
                                                                    attachment_size.set(Some(size_str));

                                                                    if let Ok(reader) = web_sys::FileReader::new() {
                                                                        let reader_clone = reader.clone();
                                                                        let mut att_data = attachment_data;
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

                                        #[cfg(not(target_arch = "wasm32"))]
                                        {
                                            let val_str = _e.value();
                                            let clean_name = val_str.rsplit(['\\', '/']).next().unwrap_or("").to_string();
                                            if !clean_name.is_empty() {
                                                attachment_name.set(Some(clean_name));
                                                attachment_size.set(Some("185 KB".to_string()));
                                            }
                                        }
                                    }
                                }
                                label {
                                    r#for: "receipt-file-input",
                                    class: "attachment-upload-label",
                                    IconPaperclip { size: "15" }
                                    span { "Pilih Foto Struk / Berkas Pembayaran" }
                                }
                            }
                        }
                    }

                    // Catatan
                    div { class: "field-group",
                        label { class: "field-label", "Catatan Keterangan" }
                        textarea {
                            class: "field-textarea",
                            placeholder: "Catatan atau keterangan tambahan...",
                            value: "{notes}",
                            oninput: move |e| notes.set(e.value()),
                        }
                    }

                    div { class: "modal-actions",
                        button {
                            r#type: "button",
                            class: "btn-secondary text-xs sm:text-sm py-2 px-3",
                            onclick: move |_| on_close.call(()),
                            "Batal"
                        }
                        button {
                            r#type: "submit",
                            class: "btn-primary text-xs sm:text-sm py-2 px-3 flex items-center justify-center gap-1.5 whitespace-nowrap",
                            IconPlus { size: "15" }
                            span { if is_edit { "Perbarui Transaksi" } else { "Simpan Transaksi" } }
                        }
                    }
                }
            }
        }
    }
}
