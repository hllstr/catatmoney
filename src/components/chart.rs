use dioxus::prelude::*;
use crate::components::icons::IconPieChart;
use crate::model::{format_idr, get_today_date, Transaction, TransactionType};
use std::collections::BTreeMap;

#[component]
pub fn CashflowChart(
    transactions: Vec<Transaction>,
    on_open_analytics: Option<EventHandler<()>>,
) -> Element {
    // Kelompokkan data transaksi berdasarkan tanggal (urutan kronologis)
    let mut daily_data: BTreeMap<String, (f64, f64)> = BTreeMap::new();

    // Pastikan tanggal hari ini terisi jika belum ada data
    let today = get_today_date();
    daily_data.insert(today.clone(), (0.0, 0.0));

    for trx in &transactions {
        let entry = daily_data.entry(trx.date.clone()).or_insert((0.0, 0.0));
        match trx.transaction_type {
            TransactionType::Income => entry.0 += trx.amount,
            TransactionType::Expense => entry.1 += trx.amount,
            TransactionType::Transfer => {
                if let Some(fee) = trx.admin_fee {
                    if fee > 0.0 {
                        entry.1 += fee;
                    }
                }
            }
        }
    }

    // Ambil maksimal 7 hari terakhir
    let items: Vec<(String, f64, f64)> = daily_data
        .into_iter()
        .map(|(date, (inc, exp))| (date, inc, exp))
        .collect();
    let display_items = if items.len() > 7 {
        &items[items.len() - 7..]
    } else {
        &items[..]
    };

    let max_val = display_items
        .iter()
        .map(|(_, inc, exp)| inc.max(*exp))
        .fold(100_000.0, f64::max);

    let chart_height = 140.0;
    let chart_width = 460.0;
    let bar_width = 14.0;
    let count = display_items.len() as f64;
    let col_width = chart_width / count.max(1.0);

    rsx! {
        div { class: "chart-container",
            div { class: "chart-header flex flex-col sm:flex-row sm:items-center justify-between gap-3",
                div {
                    h3 { class: "chart-title", "Tren Arus Kas Harian" }
                    p { class: "chart-subtitle", "Perbandingan arus masuk vs arus keluar (7 Hari Terakhir)" }
                }
                div { class: "flex items-center gap-3",
                    div { class: "chart-legend",
                        div { class: "legend-item",
                            span { class: "legend-dot income" }
                            span { "Pemasukan" }
                        }
                        div { class: "legend-item",
                            span { class: "legend-dot expense" }
                            span { "Pengeluaran" }
                        }
                    }
                    if let Some(ref handler) = on_open_analytics {
                        button {
                            r#type: "button",
                            class: "btn-secondary text-[11px] py-1 px-2.5 flex items-center gap-1.5 shrink-0",
                            onclick: {
                                let h = handler.clone();
                                move |_| h.call(())
                            },
                            IconPieChart { size: "12" }
                            span { "Analitik" }
                        }
                    }
                }
            }

            // Grafik Vektor SVG Presisi & Responsif
            div { class: "svg-chart-wrapper",
                svg {
                    view_box: "0 0 {chart_width} {chart_height + 35.0}",
                    class: "w-full h-auto overflow-visible",

                    // Garis panduan horizontal (Gridlines)
                    line { x1: "0", y1: "20", x2: "{chart_width}", y2: "20", class: "chart-gridline" }
                    line { x1: "0", y1: "{chart_height / 2.0}", x2: "{chart_width}", y2: "{chart_height / 2.0}", class: "chart-gridline" }
                    line { x1: "0", y1: "{chart_height}", x2: "{chart_width}", y2: "{chart_height}", class: "chart-baseline" }

                    // Batang Grafik per Hari
                    for (i, (date, inc, exp)) in display_items.iter().enumerate() {
                        {
                            let center_x = (i as f64 * col_width) + (col_width / 2.0);
                            let inc_h = if max_val > 0.0 { (*inc / max_val * (chart_height - 25.0)).max(2.0) } else { 2.0 };
                            let exp_h = if max_val > 0.0 { (*exp / max_val * (chart_height - 25.0)).max(2.0) } else { 2.0 };

                            let inc_y = chart_height - inc_h;
                            let exp_y = chart_height - exp_h;
                            let date_label = if date.len() >= 10 {
                                format!("{}/{}", &date[8..10], &date[5..7])
                            } else if date.len() >= 5 {
                                date[5..].to_string()
                            } else {
                                date.clone()
                            };

                            rsx! {
                                g { key: "{date}", class: "chart-group",
                                    // Batang Pemasukan
                                    rect {
                                        x: "{center_x - bar_width - 1.0}",
                                        y: "{inc_y}",
                                        width: "{bar_width}",
                                        height: "{inc_h}",
                                        rx: "3",
                                        class: "chart-bar income",
                                    }

                                    // Batang Pengeluaran
                                    rect {
                                        x: "{center_x + 1.0}",
                                        y: "{exp_y}",
                                        width: "{bar_width}",
                                        height: "{exp_h}",
                                        rx: "3",
                                        class: "chart-bar expense",
                                    }

                                    // Label Tanggal
                                    text {
                                        x: "{center_x}",
                                        y: "{chart_height + 20.0}",
                                        text_anchor: "middle",
                                        class: "chart-date-label",
                                        "{date_label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Keterangan Skala Nilai Maksimum
            div { class: "chart-scale-info tabular-numbers",
                span { "Skala Maks: {format_idr(max_val)}" }
            }
        }
    }
}
