use dioxus::prelude::*;
use crate::components::icons::{IconAlertTriangle, IconCheck, IconReceipt, IconX};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToastType {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastItem {
    pub id: u64,
    pub message: String,
    pub toast_type: ToastType,
    pub is_exiting: bool,
}

#[component]
pub fn ToastContainer(
    toasts: Vec<ToastItem>,
    on_dismiss: EventHandler<u64>,
) -> Element {
    if toasts.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "toast-container",
            for toast in toasts {
                {
                    let tid = toast.id;
                    let type_class = match toast.toast_type {
                        ToastType::Success => "toast-success",
                        ToastType::Info => "toast-info",
                        ToastType::Warning => "toast-warning",
                        ToastType::Error => "toast-error",
                    };
                    let exit_class = if toast.is_exiting { "toast-exiting" } else { "" };

                    rsx! {
                        div {
                            class: "toast-item {type_class} {exit_class}",
                            key: "{tid}",
                            div { class: "toast-icon-box",
                                match toast.toast_type {
                                    ToastType::Success => rsx! { IconCheck { size: "15" } },
                                    ToastType::Info => rsx! { IconReceipt { size: "15" } },
                                    ToastType::Warning => rsx! { IconAlertTriangle { size: "15" } },
                                    ToastType::Error => rsx! { IconAlertTriangle { size: "15" } },
                                }
                            }
                            span { class: "toast-message", "{toast.message}" }
                            button {
                                r#type: "button",
                                class: "toast-close-btn",
                                title: "Tutup Notifikasi",
                                onclick: move |_| on_dismiss.call(tid),
                                IconX { size: "13" }
                            }
                        }
                    }
                }
            }
        }
    }
}
