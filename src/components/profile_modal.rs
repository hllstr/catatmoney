use dioxus::prelude::*;
use crate::components::icons::{IconCheck, IconShield, IconUser, IconX};
use crate::model::UserProfile;

#[component]
pub fn ProfileModal(
    is_open: bool,
    current_profile: Option<UserProfile>,
    on_close: EventHandler<()>,
    on_save: EventHandler<String>,
) -> Element {
    if !is_open {
        return rsx! {};
    }

    let initial_name = current_profile.as_ref().map(|p| p.name.clone()).unwrap_or_default();
    let mut name = use_signal(move || initial_name.clone());
    let mut error_msg = use_signal(|| None::<String>);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let trimmed = name.read().trim().to_string();
        if trimmed.is_empty() {
            error_msg.set(Some("Nama panggilan tidak boleh kosong.".to_string()));
            return;
        }
        error_msg.set(None);
        on_save.call(trimmed);
        on_close.call(());
    };

    rsx! {
        div { class: "modal-backdrop", onclick: move |_| on_close.call(()),
            div {
                class: "modal-dialog max-w-md",
                onclick: move |e| e.stop_propagation(),

                div { class: "modal-header",
                    div {
                        h2 { class: "modal-title flex items-center gap-2",
                            IconUser { size: "18" }
                            "Ubah Profil Pengguna"
                        }
                        p { class: "modal-subtitle", "Perbarui nama panggilan untuk personalisasi akun CatatMoney" }
                    }
                    button {
                        r#type: "button",
                        class: "btn-icon-close",
                        onclick: move |_| on_close.call(()),
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
                        label { class: "field-label", "Nama Panggilan" }
                        input {
                            class: "field-input",
                            r#type: "text",
                            placeholder: "Misal: Budi, Sarah, Alex",
                            value: "{name}",
                            oninput: move |e| {
                                name.set(e.value());
                                error_msg.set(None);
                            },
                            autofocus: true,
                        }
                        p { class: "text-[11px] text-[var(--text-muted)] mt-1.5",
                            "Nama ini ditampilkan di banner ringkasan Dashboard harian Anda."
                        }
                    }

                    div { class: "p-3 rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface-subtle)] flex items-start gap-2.5 mb-5",
                        div { class: "text-[var(--text-muted)] mt-0.5 shrink-0",
                            IconShield { size: "15" }
                        }
                        div { class: "text-xs text-[var(--text-secondary)] leading-relaxed",
                            "Nama Anda tersimpan 100% secara privat di penyimpanan lokal peramban perangkat ini."
                        }
                    }

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
                            IconCheck { size: "14" }
                            "Simpan Perubahan"
                        }
                    }
                }
            }
        }
    }
}
