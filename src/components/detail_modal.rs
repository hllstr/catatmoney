use dioxus::prelude::*;
use crate::components::icons::{
    CategoryIcon, IconArrowDownRight, IconArrowLeftRight, IconArrowUpRight, IconCalendar, IconClock,
    IconEdit, IconEye, IconFileText, IconPaperclip, IconTrash, IconWallet, IconX,
};
use crate::model::{format_idr_privacy, Transaction, TransactionType};

#[component]
pub fn DetailModal(
    transaction: Option<Transaction>,
    #[props(default = false)] is_private: bool,
    on_close: EventHandler<()>,
    on_edit: EventHandler<Transaction>,
    on_delete: EventHandler<Transaction>,
) -> Element {
    let trx = match transaction {
        Some(t) => t,
        None => return rsx! {},
    };

    let mut show_full_preview = use_signal(|| true);

    let is_income = trx.transaction_type == TransactionType::Income;
    let is_transfer = trx.transaction_type == TransactionType::Transfer;
    let trx_for_delete = trx.clone();
    let trx_to_edit = trx.clone();
    let amount_str = if is_income {
        format!("+{}", format_idr_privacy(trx.amount, is_private))
    } else if is_transfer {
        format_idr_privacy(trx.amount, is_private)
    } else {
        format!("-{}", format_idr_privacy(trx.amount, is_private))
    };
    let admin_fee_label = match trx.admin_fee {
        Some(fee) if fee > 0.0 => format_idr_privacy(fee, is_private),
        _ => "Rp 0 (Gratis)".to_string(),
    };

    rsx! {
        // Backdrop overlay
        div { class: "modal-backdrop", onclick: move |_| on_close.call(()),
            div {
                class: "modal-dialog",
                onclick: move |e| e.stop_propagation(),

                // Header Modal
                div { class: "modal-header",
                    div {
                        h2 { class: "modal-title", "Rincian Transaksi" }
                        p { class: "modal-subtitle tabular-numbers", "ID: {trx.id}" }
                    }
                    button {
                        r#type: "button",
                        class: "btn-icon-close",
                        onclick: move |_| on_close.call(()),
                        IconX { size: "18" }
                    }
                }

                // Hero Banner Rincian Transaksi
                div {
                    class: if is_income {
                        "detail-hero-box income-theme"
                    } else if is_transfer {
                        "detail-hero-box transfer-theme"
                    } else {
                        "detail-hero-box expense-theme"
                    },
                    div { class: "flex items-center justify-between",
                        div {
                            class: if is_income {
                                "detail-type-badge positive"
                            } else if is_transfer {
                                "detail-type-badge transfer"
                            } else {
                                "detail-type-badge negative"
                            },
                            if is_income {
                                IconArrowUpRight { size: "14" }
                                span { "Pemasukan" }
                            } else if is_transfer {
                                IconArrowLeftRight { size: "14" }
                                span { "Transfer Antar Akun" }
                            } else {
                                IconArrowDownRight { size: "14" }
                                span { "Pengeluaran" }
                            }
                        }
                    }
                    div { class: "mt-2.5",
                        h3 {
                            class: if is_income {
                                "detail-amount-display income tabular-numbers"
                            } else if is_transfer {
                                "detail-amount-display transfer tabular-numbers"
                            } else {
                                "detail-amount-display expense tabular-numbers"
                            },
                            "{amount_str}"
                        }
                        p { class: "detail-title-display", "{trx.title}" }
                    }
                }

                // Grid Metadata Transaksi
                div { class: "detail-grid",
                    // 1. Kategori
                    div { class: "detail-grid-item",
                        span { class: "detail-item-label", "Kategori" }
                        div { class: "flex items-center gap-2 mt-1",
                            div { class: "w-6 h-6 rounded-md bg-[var(--bg-card)] border border-[var(--border-subtle)] flex items-center justify-center shrink-0",
                                if is_transfer {
                                    IconArrowLeftRight { size: "14" }
                                } else {
                                    CategoryIcon { category: trx.category.clone() }
                                }
                            }
                            span { class: "font-semibold text-sm truncate", "{trx.category}" }
                        }
                    }

                    // 2. Sumber Dana / Akun Asal
                    div { class: "detail-grid-item",
                        span { class: "detail-item-label", if is_transfer { "Akun Asal" } else { "Sumber Dana" } }
                        div { class: "flex items-center gap-1.5 mt-1 font-semibold text-sm",
                            IconWallet { size: "14" }
                            span { class: "truncate", "{trx.wallet}" }
                        }
                    }

                    // Tambahan untuk Transfer Antar Akun
                    if is_transfer {
                        div { class: "detail-grid-item",
                            span { class: "detail-item-label", "Akun Tujuan" }
                            div { class: "flex items-center gap-1.5 mt-1 font-semibold text-sm",
                                IconWallet { size: "14" }
                                span { class: "truncate", "{trx.to_wallet.as_deref().unwrap_or(\"-\")}" }
                            }
                        }
                        div { class: "detail-grid-item",
                            span { class: "detail-item-label", "Biaya Admin" }
                            div { class: "flex items-center gap-1.5 mt-1 font-semibold text-sm tabular-numbers",
                                span { "{admin_fee_label}" }
                            }
                        }
                    }

                    // 3. Tanggal Transaksi
                    div { class: "detail-grid-item",
                        span { class: "detail-item-label", "Tanggal" }
                        div { class: "flex items-center gap-1.5 mt-1 font-semibold text-sm tabular-numbers",
                            IconCalendar { size: "14" }
                            span { "{trx.date}" }
                        }
                    }

                    // 4. Waktu Presisi
                    div { class: "detail-grid-item",
                        span { class: "detail-item-label", "Waktu Pencatatan" }
                        div { class: "flex items-center gap-1.5 mt-1 font-semibold text-sm tabular-numbers",
                            IconClock { size: "14" }
                            span { "{trx.time} WIB" }
                        }
                    }
                }

                // Catatan Keterangan
                div { class: "detail-section",
                    span { class: "detail-item-label", "Catatan Keterangan" }
                    if trx.notes.is_empty() {
                        p { class: "text-xs text-muted italic mt-1", "Tidak ada catatan tambahan." }
                    } else {
                        div { class: "detail-notes-box",
                            p { "{trx.notes}" }
                        }
                    }
                }

                // Bagian Lampiran Bukti / Struk (Attachment Preview)
                div { class: "detail-section",
                    div { class: "flex items-center justify-between mb-1.5",
                        span { class: "detail-item-label flex items-center gap-1.5",
                            IconPaperclip { size: "12" }
                            span { "Bukti / Struk Pembayaran" }
                        }
                        if trx.attachment_data.is_some() || trx.attachment_name.is_some() {
                            button {
                                r#type: "button",
                                class: "btn-link text-xs flex items-center gap-1 cursor-pointer",
                                onclick: move |_| {
                                    let curr = *show_full_preview.read();
                                    show_full_preview.set(!curr);
                                },
                                IconEye { size: "13" }
                                if *show_full_preview.read() { "Sembunyikan" } else { "Lihat" }
                            }
                        }
                    }

                    if let Some(ref file_name) = trx.attachment_name {
                        div { class: "attachment-card flex-col items-stretch gap-2.5",
                            div { class: "flex items-center gap-3",
                                div { class: "attachment-icon-badge",
                                    IconPaperclip { size: "16" }
                                }
                                div { class: "flex flex-col",
                                    span { class: "attachment-file-name", "{file_name}" }
                                    if let Some(ref sz) = trx.attachment_size {
                                        span { class: "attachment-file-size", "{sz}" }
                                    }
                                }
                            }

                            // Pratinjau Visual Langsung Gambar / Struk dengan Fallback Cerdas
                            if *show_full_preview.read() {
                                if let Some(ref data_uri) = trx.attachment_data {
                                    if data_uri.starts_with("data:image/") || data_uri.starts_with("http://") || data_uri.starts_with("https://") {
                                        div { class: "receipt-visual-preview-container",
                                            img {
                                                src: "{data_uri}",
                                                alt: "{file_name}",
                                                class: "receipt-visual-preview-image",
                                            }
                                        }
                                    } else {
                                        div { class: "receipt-mock-preview-container",
                                            div { class: "flex flex-col items-center justify-center p-6 text-center",
                                                IconFileText { size: "28" }
                                                span { class: "font-bold text-sm mt-2 text-[var(--text-primary)]", "{file_name}" }
                                                if let Some(ref sz) = trx.attachment_size {
                                                    span { class: "text-xs text-[var(--text-muted)] mt-1", "{sz}" }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    div { class: "receipt-mock-preview-container",
                                        div { class: "flex flex-col items-center justify-center p-6 text-center",
                                            IconFileText { size: "28" }
                                            span { class: "font-bold text-sm mt-2 text-[var(--text-primary)]", "{file_name}" }
                                            if let Some(ref sz) = trx.attachment_size {
                                                span { class: "text-xs text-[var(--text-muted)] mt-1", "{sz}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "attachment-empty-box",
                            p { "Tidak ada berkas bukti pembayaran yang dilampirkan pada transaksi ini." }
                        }
                    }
                }

                // Actions Footer
                div { class: "detail-actions-footer",
                    button {
                        r#type: "button",
                        class: "btn-primary text-xs py-2 px-3 flex items-center justify-center gap-1.5",
                        onclick: {
                            let t = trx_to_edit.clone();
                            move |_| {
                                on_edit.call(t.clone());
                                on_close.call(());
                            }
                        },
                        IconEdit { size: "14" }
                        "Edit Transaksi"
                    }
                    button {
                        r#type: "button",
                        class: "btn-danger-outline text-xs py-2 px-3 flex items-center justify-center gap-1.5",
                        onclick: move |_| {
                            on_delete.call(trx_for_delete.clone());
                            on_close.call(());
                        },
                        IconTrash { size: "14" }
                        "Hapus"
                    }
                }
            }
        }
    }
}
