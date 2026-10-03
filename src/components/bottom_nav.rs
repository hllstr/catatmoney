use dioxus::prelude::*;
use crate::components::icons::{
    IconCalendar, IconLayoutDashboard, IconList, IconPieChart, IconPiggyBank, IconPlus, IconSliders,
    IconTarget,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NavTab {
    Dashboard,
    Analytics,
    Budget,
    Savings,
    Calendar,
    History,
    Management,
}

#[component]
pub fn BottomNavBar(
    active_tab: NavTab,
    on_select_tab: EventHandler<NavTab>,
    on_open_modal: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "bottom-dock-wrapper",
            nav { class: "bottom-dock",
                // Tab Dashboard
                button {
                    r#type: "button",
                    title: "Dashboard",
                    aria_label: "Dashboard",
                    class: if active_tab == NavTab::Dashboard { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Dashboard),
                    IconLayoutDashboard { size: "18" }
                    span { class: "dock-label", "Dashboard" }
                }

                // Tab Analitik Keuangan
                button {
                    r#type: "button",
                    title: "Analitik Keuangan",
                    aria_label: "Analitik Keuangan",
                    class: if active_tab == NavTab::Analytics { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Analytics),
                    IconPieChart { size: "18" }
                    span { class: "dock-label", "Analitik" }
                }

                // Tab Anggaran & Batas Belanja Bulanan
                button {
                    r#type: "button",
                    title: "Anggaran & Batas Belanja",
                    aria_label: "Anggaran & Batas Belanja",
                    class: if active_tab == NavTab::Budget { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Budget),
                    IconTarget { size: "18" }
                    span { class: "dock-label", "Anggaran" }
                }

                // Tab Target Tabungan & Finansial Impian (Baru)
                button {
                    r#type: "button",
                    title: "Target Tabungan",
                    aria_label: "Target Tabungan",
                    class: if active_tab == NavTab::Savings { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Savings),
                    IconPiggyBank { size: "18" }
                    span { class: "dock-label", "Tabungan" }
                }

                // Tombol Aksi Tambah Transaksi Utama (+) di Tengah
                div { class: "dock-fab-container",
                    button {
                        r#type: "button",
                        class: "dock-fab-btn",
                        title: "Tambah Catatan Uang",
                        aria_label: "Tambah Catatan Uang",
                        onclick: move |_| on_open_modal.call(()),
                        IconPlus { size: "22" }
                    }
                }

                // Tab Kalender Keuangan
                button {
                    r#type: "button",
                    title: "Kalender Keuangan",
                    aria_label: "Kalender Keuangan",
                    class: if active_tab == NavTab::Calendar { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Calendar),
                    IconCalendar { size: "18" }
                    span { class: "dock-label", "Kalender" }
                }

                // Tab Riwayat Transaksi
                button {

                    r#type: "button",
                    title: "Riwayat Transaksi",
                    aria_label: "Riwayat Transaksi",
                    class: if active_tab == NavTab::History { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::History),
                    IconList { size: "18" }
                    span { class: "dock-label", "Riwayat" }
                }

                // Tab Manajemen / Kelola Kategori & Sumber Dana
                button {
                    r#type: "button",
                    title: "Kelola Kategori & Sumber Dana",
                    aria_label: "Kelola Kategori & Sumber Dana",
                    class: if active_tab == NavTab::Management { "dock-item active" } else { "dock-item" },
                    onclick: move |_| on_select_tab.call(NavTab::Management),
                    IconSliders { size: "18" }
                    span { class: "dock-label", "Kelola" }
                }
            }
        }
    }
}
