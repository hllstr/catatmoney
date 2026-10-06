use dioxus::prelude::*;
use crate::audio::sleep_ms;
use crate::components::icons::{IconCheck, IconTag, IconX};

#[component]
pub fn CategoryModal(
    is_open: bool,
    is_expense: bool,
    #[props(default = None)] initial_name: Option<String>,
    #[props(default = Vec::new())] existing_categories: Vec<String>,
    on_close: EventHandler<()>,
    on_save: EventHandler<String>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let is_edit = initial_name.is_some();
    let label_kind = if is_expense { "Pengeluaran" } else { "Pemasukan" };
    let initial_val = initial_name.clone().unwrap_or_default();
    let initial_val_check = initial_val.clone();
    let mut name = use_signal(move || initial_val.clone());
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
        let existing_categories = existing_categories.clone();
        move |evt: FormEvent| {
            evt.prevent_default();
            let trimmed = name.read().trim().to_string();
            if trimmed.is_empty() {
                error_msg.set(Some("Nama kategori tidak boleh kosong.".to_string()));
                return;
            }

            let is_duplicate = if is_edit {
                !trimmed.eq_ignore_ascii_case(&initial_val_check)
                    && existing_categories.iter().any(|c| c.eq_ignore_ascii_case(&trimmed))
            } else {
                existing_categories.iter().any(|c| c.eq_ignore_ascii_case(&trimmed))
            };

            if is_duplicate {
                error_msg.set(Some("Kategori dengan nama tersebut sudah ada.".to_string()));
                return;
            }

            error_msg.set(None);
            if *is_closing.read() {
                return;
            }
            is_closing.set(true);
            let on_save = on_save.clone();
            let on_close = on_close.clone();
            spawn(async move {
                on_save.call(trimmed);
                sleep_ms(220).await;
                is_closing.set(false);
                on_close.call(());
            });
        }
    };

    let modal_title = if is_edit {
        format!("Edit Kategori {}", label_kind)
    } else {
        format!("Tambah Kategori {} Baru", label_kind)
    };
    let modal_subtitle = format!("Kategori yang digunakan untuk klasifikasi transaksi {}", label_kind.to_lowercase());
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
                            IconTag { size: "18" }
                            "{modal_title}"
                        }
                        p { class: "modal-subtitle",
                            "{modal_subtitle}"
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
                    div { class: "field-group mb-4",
                        label { class: "field-label", "Nama Kategori" }
                        input {
                            class: "field-input",
                            r#type: "text",
                            placeholder: if is_expense { "Misal: Langganan SaaS, Pajak, Hobi" } else { "Misal: Dividen, Saham, Hibah" },
                            value: "{name}",
                            oninput: move |e| {
                                name.set(e.value());
                                error_msg.set(None);
                            },
                            autofocus: true,
                        }
                    }

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
                            IconCheck { size: "14" }
                            if is_edit { "Simpan Perubahan" } else { "Simpan Kategori" }
                        }
                    }
                }
            }
        }
    }
}
