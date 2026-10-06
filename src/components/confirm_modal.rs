use dioxus::prelude::*;
use crate::audio::sleep_ms;
use crate::components::icons::{IconAlertTriangle, IconTrash, IconX};

#[component]
pub fn ConfirmModal(
    title: String,
    message: String,
    #[props(default = "Hapus Data".to_string())] confirm_label: String,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut is_closing = use_signal(|| false);

    let handle_close = {
        let on_cancel = on_cancel.clone();
        move || {
            if *is_closing.read() {
                return;
            }
            is_closing.set(true);
            spawn(async move {
                sleep_ms(220).await;
                is_closing.set(false);
                on_cancel.call(());
            });
        }
    };

    let handle_confirm_action = {
        let on_confirm = on_confirm.clone();
        move || {
            if *is_closing.read() {
                return;
            }
            is_closing.set(true);
            spawn(async move {
                sleep_ms(220).await;
                is_closing.set(false);
                on_confirm.call(());
            });
        }
    };

    let closing_class = if *is_closing.read() { "modal-closing" } else { "" };

    rsx! {
        div {
            class: "modal-backdrop modal-backdrop-center {closing_class}",
            onclick: {
                let mut close_fn = handle_close.clone();
                move |_| close_fn()
            },
            div {
                class: "modal-dialog max-w-md {closing_class}",
                onclick: move |e| e.stop_propagation(),

                // Header Dialog Konfirmasi
                div { class: "flex items-start justify-between pb-4 border-b border-[var(--border-subtle)]",
                    div { class: "flex items-center gap-3",
                        div { class: "p-2 rounded-lg bg-[var(--negative-bg)] text-[var(--negative)] border border-[var(--negative-border)] flex items-center justify-center shrink-0",
                            IconAlertTriangle { size: "20" }
                        }
                        div {
                            h3 { class: "text-base font-bold text-[var(--text-primary)] leading-tight", "{title}" }
                            p { class: "text-xs text-[var(--text-muted)] mt-0.5", "Tindakan ini memerlukan verifikasi Anda" }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "modal-close-btn",
                        onclick: {
                            let mut close_fn = handle_close.clone();
                            move |_| close_fn()
                        },
                        IconX { size: "18" }
                    }
                }

                // Pesan Deskripsi Konfirmasi
                div { class: "py-5",
                    p { class: "text-sm text-[var(--text-secondary)] leading-relaxed",
                        "{message}"
                    }
                }

                // Aksi Tombol
                div { class: "flex items-center justify-end gap-3 pt-3 border-t border-[var(--border-subtle)]",
                    button {
                        r#type: "button",
                        class: "btn-secondary text-xs px-4 py-2",
                        onclick: {
                            let mut close_fn = handle_close.clone();
                            move |_| close_fn()
                        },
                        "Batal"
                    }
                    button {
                        r#type: "button",
                        class: "btn-danger text-xs px-4 py-2 flex items-center gap-1.5",
                        onclick: {
                            let mut confirm_fn = handle_confirm_action.clone();
                            move |_| confirm_fn()
                        },
                        IconTrash { size: "14" }
                        "{confirm_label}"
                    }
                }
            }
        }
    }
}
