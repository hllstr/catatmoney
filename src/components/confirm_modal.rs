use dioxus::prelude::*;
use crate::components::icons::{IconAlertTriangle, IconTrash, IconX};

#[component]
pub fn ConfirmModal(
    title: String,
    message: String,
    #[props(default = "Hapus Data".to_string())] confirm_label: String,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "modal-backdrop modal-backdrop-center",
            onclick: move |_| on_cancel.call(()),
            div {
                class: "modal-dialog max-w-md",
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
                        onclick: move |_| on_cancel.call(()),
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
                        onclick: move |_| on_cancel.call(()),
                        "Batal"
                    }
                    button {
                        r#type: "button",
                        class: "btn-danger text-xs px-4 py-2 flex items-center gap-1.5",
                        onclick: move |_| on_confirm.call(()),
                        IconTrash { size: "14" }
                        "{confirm_label}"
                    }
                }
            }
        }
    }
}
